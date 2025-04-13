use clap::Parser;
use cli::{Cli, Commands};
use mqtt::{publish, subscribe};
use std::{process::exit, time::Duration};
use tokio::main;

pub mod cli;
pub mod mqtt;

const ERR_PARSING_ARGS: i32 = 1;
const ERR_CREATING_MQTT_CLIENT: i32 = 2;
const ERR_FAILED_COMMAND: i32 = 3;

#[main]
async fn main() {
    let cli = Cli::try_parse().unwrap_or_else(|e| {
        eprintln!("Error parsing arguments: {}", e);
        exit(ERR_PARSING_ARGS);
    });

    // Build Client Config and the MQTT client used
    let config = mqtt::ClientConfig::new(
        cli.host.unwrap_or_else(|| "localhost".to_string()),
        cli.port.unwrap_or(1883),
    );
    let mut client = mqtt::build_client(config).unwrap_or_else(|e| {
        eprintln!("Error creating MQTT client: {}", e);
        exit(ERR_CREATING_MQTT_CLIENT);
    });

    let topic = cli.topic;
    let qos = cli.qos;

    match cli.command {
        Commands::Pub { message } => publish(&client, topic, &message, qos).await,
        Commands::Sub {} => subscribe(&mut client, topic, qos).await,
    }
    .unwrap_or_else(|cmd_err| {
        eprintln!("{}", cmd_err.to_string());
        exit(ERR_FAILED_COMMAND);
    });
}
