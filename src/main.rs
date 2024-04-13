use std::time::Duration;
use sysinfo::System;
use discord_rpc_client::Client;
use tokio::{task, time};

const CLIENT_KEY: u64 = 1228513276720058388; // replace this value with the client key for your Discord application
const EXECUTABLE_NAME: &str = "FL64.exe";

#[tokio::main]
async fn main() {
    // Create a new Discord RPC client
    let mut drpc = Client::new(CLIENT_KEY);

    drpc.on_ready(|_ctx| {
        println!("FL Studio rich presence loaded with client ID {}", CLIENT_KEY);
    });

    // start the RPC client
    drpc.start();

    let s = System::new_all();

    // start the task to update presence
    let update_process = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            // Find the process by name and check to see if FL Studio is running
            let _process = s.processes_by_name(EXECUTABLE_NAME)
                .next()
                .unwrap_or_else(|| {
                    eprintln!("No DAW processes found.");
                    std::process::exit(1);
                });

            let process_name = "FL Studio";

            // let runtime: u64 = process.run_time();

            // Set activity
            if let Err(msg) = drpc.set_activity(|a| a
                .state(process_name)
                .assets(|asset| 
                    asset.large_image("pon3")
                        .large_text("dj pon3")) // hehehehe :3 :3 :3
            )
            {
                eprintln!("Failed to set presence: {}", msg)
            }
        }
    });

    // Await the completion of the update process task
    if let Err(err) = update_process.await {
        eprintln!("Failed to await update process task: {}", err);
    }
}
