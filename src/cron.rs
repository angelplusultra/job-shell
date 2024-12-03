use std::error::Error;

use tokio_cron_scheduler::{Job as CronJob, JobScheduler};

use crate::{models::data::Data, handle_scan_new_jobs_across_network};

pub async fn initialize_cron() -> Result<(), Box<dyn Error>> {
    // Create a new scheduler
    let scheduler = JobScheduler::new().await?;

    // Create a job that runs every 5 minutes
    let job1 = CronJob::new_async("0 0 */6 * * *", move |_uuid, _lock| {
        Box::pin(async move {
            let mut data = Data::get_data();
            if let Err(e) = handle_scan_new_jobs_across_network(&mut data).await {
                eprintln!("Error scanning for jobs: {}", e);
            }
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
