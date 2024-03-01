use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn initialize_scheduler() -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    // Start the scheduler
    sched.start().await?;

    // Add basic cron job
    sched
        .add(Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        })?)
        .await?;

    //Wait while the jobs run
    //tokio::time::sleep(Duration::from_secs(100)).await;
    println!("Scheduler Initialized");
    Ok(())
}
