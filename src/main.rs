use std::time::Duration;

use sysinfo::System;
use discord_rpc_client::Client;
use tokio::{
    task,
    time,
};

const CLIENT_KEY: u64 = 1228513276720058388; // replace this value with the client key for your discord application

#[tokio::main]
async fn main() {
    // create a new discord RPC client
    let mut drpc = Client::new(CLIENT_KEY);

    drpc.on_ready(|_ctx| {
        println!("FL Studio rich presence loaded with client ID {}", CLIENT_KEY);
    });

    drpc.start(); // start the rpc client

    let s = System::new_all();

    // todo: add the image for FL studio to the rich presence
    // todo: add external interaction with winapi to view window titles
    
    let update_process = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            
            let runtime: u64;
        
            let process_name: &str = "FL Studio";

            // find DAW instances
            let process = s.processes_by_name("FL64")
                .next()
                .expect("No DAW processes found.");
            runtime = process.run_time();

            if let Err(msg) = drpc.set_activity(|a| a
                        .state(process_name)
                        .assets(|assets| assets.large_text(runtime.to_string()))) {
                println!("! Failed to set presence: {}", msg)
            }
        }
    });

    let _ = update_process.await;
}
