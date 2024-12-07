use std::error::Error;

use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};
use tokio_cron_scheduler::{Job as CronJob, JobScheduler};

use crate::{
    models::{data::Data, scraper::JobsPayload},
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
) -> Result<(), Box<dyn Error>> {
    // Create a new scheduler
    let scheduler = JobScheduler::new().await?;

    let every_interval_by_hours = format!("0 0 */{} * * *", cron_interval);
    let every_interval_by_minutes = format!("0 */{} * * * *", cron_interval);
    // Create a job that runs every 5 minutes
    let job1 = CronJob::new_async(every_interval_by_minutes, move |uuid, mut lock| {
        let webhook_url = webhook_url.clone();
        Box::pin(async move {
            let next_tick = lock.next_tick_for_job(uuid).await;
            match next_tick {
                Ok(Some(ts)) => println!("Next time for the job is {:?}", ts),
                _ => println!("Could not get next tick for 7s job"),
            }

            println!("Discord cron starting!");
            let total_new_jobs = scan_for_new_jobs().await;

            clear_console();

            if total_new_jobs.is_empty() {
                println!("No new jobs detected");
                return;
            }
            println!("Finished Scraping");
            println!("Building messages and sending to Discord");

            deploy_messages_to_discord(total_new_jobs, webhook_url).await;

            println!("Process finished!");
        })
    })?;

    // Add job to the scheduler
    scheduler.add(job1).await?;

    // Start the scheduler
    scheduler.start().await?;

    println!("Job scheduler started! Press Ctrl+C to exit.");

    // Keep the main task running
    tokio::signal::ctrl_c().await?;
    println!("Shutting down scheduler...");

    Ok(())
}

fn calculate_messages_and_embeds(
    total_entries: usize,
    entries_per_embed: usize,
    embeds_per_message: usize,
) -> (usize, usize) {
    // Calculate total embeds needed (rounding up)
    let total_embeds = (total_entries + entries_per_embed - 1) / entries_per_embed;

    // Calculate total messages needed (rounding up)
    let total_messages = (total_embeds + embeds_per_message - 1) / embeds_per_message;

    (total_messages, total_embeds)
}

async fn scan_for_new_jobs() -> Vec<FormattedJob> {
    let mut data = Data::get_data();
    let company_keys: Vec<String> = data.data.keys().cloned().collect();

    let mut total_new_jobs: Vec<FormattedJob> = Vec::new();
    for key in &company_keys {
        println!("Scanning new jobs @ {key}");
        let jobs_payload_result = scrape_jobs(&mut data, &key).await;

        match jobs_payload_result {
            Ok(jobs_payload) => {
                if jobs_payload.are_new_jobs {
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
            Err(e) => {
                eprintln!("Error scanning new jobs for {key}\nError: {e}");
            }
        }
    }

    total_new_jobs
}

async fn deploy_messages_to_discord(total_new_jobs: Vec<FormattedJob>, webhook_url: String) {
    let entries_per_embed = 15;
    let embeds_per_message = 10;
    let (total_messages, total_embeds) =
        calculate_messages_and_embeds(total_new_jobs.len(), entries_per_embed, embeds_per_message);

    println!(
        "Total Messages: {}\nTotal Embeds: {}\nTotal New Jobs: {}",
        total_messages,
        total_embeds,
        total_new_jobs.len()
    );

    for _ in 0..total_messages {
        let mut message = Message {
                    username: "JobShell".to_string(),
                    avatar_url: "https://cdn.discordapp.com/attachments/917180495849197568/1305854030899314688/jobshell_icon.png?ex=67538616&is=67523496&hm=a7dfa93aaf187bc3c791ed5a8622fa4769b5cfef186524f174cc0cf6e8b3498c&".to_string(),
                    embeds: Vec::new(),
                };
        for embed_set in total_new_jobs.chunks(15) {
            let mut embed = Embed {
                title: "New Jobs".to_string(),
                fields: Vec::new(),
            };
            for job in embed_set {
                let field = Field {
                    name: format!("{} at {}", job.title, job.company),
                    value: format!("Location: {}\n[Apply here]({})", job.location, job.link),
                    inline: false,
                };

                embed.fields.push(field);
            }
            message.embeds.push(embed);
        }
    }

    // for _ in 0..total_messages {
    //     let mut message = Message {
    //                 username: "JobShell".to_string(),
    //                 avatar_url: "https://cdn.discordapp.com/attachments/917180495849197568/1305854030899314688/jobshell_icon.png?ex=67538616&is=67523496&hm=a7dfa93aaf187bc3c791ed5a8622fa4769b5cfef186524f174cc0cf6e8b3498c&".to_string(),
    //                 embeds: Vec::new(),
    //             };
    //
    //     let mut char_count = 0;
    //     for _ in 0..embeds_per_message {
    //         if current_job_index >= total_new_jobs.len() {
    //             break;
    //         }
    //
    //         let mut embed = Embed {
    //             title: "Jobs Update".to_string(),
    //             fields: Vec::new(),
    //         };
    //
    //         for _ in 0..entries_per_embed {
    //             if current_job_index >= total_new_jobs.len() {
    //                 break;
    //             }
    //
    //             let job = &total_new_jobs[current_job_index];
    //
    //             embed.fields.push(Field {
    //                 name: format!("{} at {}", job.title, job.company),
    //                 value: format!("Location: {}\n[Apply here]({})", job.location, job.link),
    //                 inline: false,
    //             });
    //
    //             current_job_index += 1;
    //         }
    //
    //         message.embeds.push(embed);
    //     }
    //
    //     let res = Client::new().post(&webhook_url).json(&message).send().await;
    //
    //     match res {
    //         Ok(res) => match res.text().await {
    //             Err(e) => {
    //                 eprintln!("An error occured streaming the response to text: {e}")
    //             }
    //             Ok(text) => println!("Success!\n{text}"),
    //         },
    //         Err(e) => {
    //             eprintln!("An error occured sedning the POST to request to the webhook URL: {e}")
    //         }
    //     }
    // }
}
