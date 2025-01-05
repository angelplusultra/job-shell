use std::error::Error;

use anyhow::anyhow;
use reqwest::Client;
use serde::Serialize;
use tokio_cron_scheduler::{Job as CronJob, JobScheduler};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        ai::{AiModel, OpenAIClient},
        data::Data,
        scraper::Job,
    },
    scrape_jobs,
    utils::clear_console,
};

#[derive(Serialize, Debug)]
struct FormattedJob {
    title: String,
    location: String,
    link: String,
    company: String,
}

#[derive(Serialize)]
struct Message {
    username: String,
    avatar_url: String,
    embeds: Vec<Embed>,
}

#[derive(Serialize)]
struct Embed {
    title: String,
    fields: Vec<Field>,
}

#[derive(Serialize)]
struct Field {
    name: String,
    value: String,
    inline: bool,
}

pub async fn initialize_discord_mode(
    webhook_url: String,
    cron_interval: u64,
    scan_all_companies: bool,
) -> AppResult<()> {
    let scheduler = JobScheduler::new().await?;

    let cron = format!("every {} hours", cron_interval);
    println!("Using cron expression: {}", cron);

    let job = CronJob::new_async(cron, move |uuid, mut lock| {
        let webhook_url = webhook_url.clone();
        Box::pin(async move {
            println!("Discord cron starting!");

            let total_new_jobs = scan_for_new_jobs(scan_all_companies).await;

            if total_new_jobs.is_empty() {
                println!("No new jobs detected");
                return; // Return Ok for successful empty check
            }

            println!("Finished Scraping");
            println!("Building messages and sending to Discord");

            deploy_messages_to_discord(total_new_jobs, webhook_url, 2).await;

            println!("Process finished!");

            if let Ok(Some(ts)) = lock.next_tick_for_job(uuid).await {
                println!("Next run scheduled for: {:?}", ts);
            } else {
                println!("Could not determine next run time");
            }
        })
    })?;

    // Add job to the scheduler
    scheduler.add(job).await?;

    // Start the scheduler
    scheduler.start().await?;
    println!("Job scheduler started! Press Ctrl+C to exit.");

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    println!("Shutting down scheduler...");

    Ok(())
}

async fn scan_for_new_jobs(scan_all_companies: bool) -> Vec<FormattedJob> {
    let mut data = Data::get_data();
    let mut company_keys: Vec<String> = data.companies.keys().cloned().collect();

    if !scan_all_companies {
        company_keys.retain(|k| {
            data.companies[k].is_following || !data.companies[k].connections.is_empty()
        });
    }

    let mut total_new_jobs: Vec<FormattedJob> = Vec::new();
    for key in &company_keys {
        println!("Scanning new jobs @ {key}");

        let jobs_payload_result = scrape_jobs(&mut data, &key).await;

        match jobs_payload_result {
            Ok(jobs_payload) => {
                if jobs_payload.are_new_jobs {
                    if data.smart_criteria_enabled {
                        println!("Filtering jobs based on smart criteria");
                        let openai_client = OpenAIClient::new();
                        let result = openai_client
                            .filter_jobs_based_on_smart_criteria(&jobs_payload.new_jobs)
                            .await;

                        match result {
                            Ok(filtered_jobs) => {
                                let formatted_jobs = filtered_jobs
                                    .iter()
                                    .map(|j| FormattedJob {
                                        title: j.title.clone(),
                                        link: j.link.clone(),
                                        location: j.location.clone(),
                                        company: key.to_owned(),
                                    })
                                    .collect::<Vec<FormattedJob>>();
                                total_new_jobs.extend(formatted_jobs);
                            }
                            Err(e) => {
                                eprintln!("Error filtering jobs for {key}\nError: {e}");
                            }
                        }
                    } else {
                        let formatted_jobs = jobs_payload
                            .new_jobs
                            .iter()
                            .map(|j| FormattedJob {
                                title: j.title.clone(),
                                link: j.link.clone(),
                                location: j.location.clone(),
                                company: key.to_owned(),
                            })
                            .collect::<Vec<FormattedJob>>();
                        total_new_jobs.extend(formatted_jobs);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error scanning new jobs for {key}\nError: {e}");
            }
        }
    }

    total_new_jobs
}

async fn deploy_messages_to_discord(
    total_new_jobs: Vec<FormattedJob>,
    webhook_url: String,
    embeds_per_message: usize,
) {
    let embeds: Vec<&[FormattedJob]> = total_new_jobs.chunks(15).collect();

    let messages = embeds.chunks(embeds_per_message);

    println!("Number of messages: {}", messages.len());
    for message_set in messages {
        let mut new_message = Message {
            username: "Jobshell".to_string(),
                    avatar_url: "https://cdn.discordapp.com/attachments/917180495849197568/1305854030899314688/jobshell_icon.png?ex=67538616&is=67523496&hm=a7dfa93aaf187bc3c791ed5a8622fa4769b5cfef186524f174cc0cf6e8b3498c&".to_string(),
            embeds: Vec::new(),
        };

        for embed in message_set.iter() {
            let mut new_embed = Embed {
                title: "New Jobs".to_string(),
                fields: Vec::new(),
            };
            for job in embed.iter() {
                let field = Field {
                    name: format!("{} @ {}", job.title, job.company),
                    value: format!("{}\n[Apply Here]({})", job.location, job.link),
                    inline: false,
                };

                new_embed.fields.push(field);
            }

            new_message.embeds.push(new_embed);
        }

        let res = Client::new()
            .post(&webhook_url)
            .json(&new_message)
            .send()
            .await
            .unwrap();

        if res.status().is_success() {
            println!("Message sent!");
        }
    }
}
