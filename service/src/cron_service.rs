use crate::nation_service::NationMutation;
use log::{error, info};
use tokio_cron_scheduler::{Job, JobScheduler};

// every second
// 0 * * * * * *" never runs
// 1 * * * * * *" every first second
// "0/1 * * * * * *" first of one second
// "1/1 * * * * * *" first of one second, same as 0/1

// every other second
// "0/2 * * * * * *" first of every other second
// "1/2 * * * * * *" second of every other second
// "2/2 * * * * * *" same as first of every other second

// every 10 seconds
// "0/10 * * * * * *" first of every 10 seconds
// "5/10 * * * * * *" fifth of every 10 seconds
// "10/10 * * * * * *" first of every 10 seconds, same as the first

// every minute
// "0 0/1 * * * * *" // at first second
// "0 0/1 * * * * *" // at first second, same as above
// "10 0/1 * * * * *" // at 10 seconds in
// "1/10,3/10 0/1 * * *  * *" // every first and third second, every ten seconds

//every ten minutes
// "0 0/10 * * * * *" // every first second on every tenth minute
// "* 0/10 * * * * *" // every second on every tenth minute
// "1/10,3/10 0/10 * * *  * *" // every first and third second, every ten seconds during the first minute of ten minutes

// every hour
// "0 1 * * * * *" // every hour on the first minute of the hour - confirmed!

pub async fn initialize_scheduler() -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    // Start the scheduler
    sched.start().await?;

    // Income Job - Every 1 Minute
    sched
        .add(Job::new_async("0/2 * * * * * *", |_uuid, _l| {
            Box::pin(async move {
                println!("I run every minute");

                // calculate income
                let update_future = NationMutation::update_gold_from_income_timer().await;

                match update_future {
                    Ok(_) => {
                        println!("");
                        info!("Gold update job 'update_gold_from_income_timer' was successful!")
                    }
                    Err(error) => {
                       print!("{error}");
                        error!("Something went wrong in the job 'update_gold_from_income_timer' : {error}");
                    }
                }
            })
        })?)
        .await?;

    // Upkeep Job - Every 5 Minute
    sched
        .add(Job::new_async("0 0/5 * * * * *", |_uuid, _l| {
            Box::pin(async move {
                println!("I run every 5 minutes");
                let update_future = NationMutation::update_gold_from_upkeep_timer().await;

                match update_future {
                    Ok(_) => {
                        info!("Gold update job 'update_gold_from_income_timer' was successful!")
                    }
                    Err(error) => {
                        // print!("{error}");
                        error!("Something went wrong in the job 'update_gold_from_upkeep_timer' : {error}");
                    }
                }
            })
        })?)
        .await?;
    println!("Scheduler Initialized");
    Ok(())
}
