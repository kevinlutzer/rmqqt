use clap::Parser;
use cli::{Cli, Commands};
use mqtt::{build_client, connect_client, publish, subscribe, ClientConfig};

#[allow(deprecated)]
use std::{env::home_dir, process::exit};
use tokio::main;

pub mod cli;
pub mod mqtt;

const CONFIG_FILE_NAME: &str = ".rmqttconfig";

/// Exit code when the program fails to parse the arguments. This means that the user
/// provided invalid arguments or the arguments were not provided at all.
const ERR_PARSING_ARGS: i32 = 1;
/// Exit code when the program fails to create the MQTT client. This means that the
/// MQTT client could not be created due to invalid configuration or other errors.
const ERR_CREATING_MQTT_CLIENT: i32 = 2;
/// Exit code when the program fails to connect to the MQTT broker. This means that
/// the MQTT client could not connect to the broker because it might not exist or the configuration
/// is invalid.
const ERR_CONNECTING_MQTT_BROKER: i32 = 3;
/// Exit code when the program fails to execute the command. This means that either publishing
/// or subscribing to the topic failed.
const ERR_FAILED_COMMAND: i32 = 4;

/// TODO - add support for Windows home directory and SNAP_COMMON in the future
fn load_config_file() {
    // Get a path ref to the config file
    #[allow(deprecated)]
    let some_config_file = home_dir().map(|path| path.join(CONFIG_FILE_NAME));

    // If the config file exists, load the environment variables from it
    if let Some(config_file) = some_config_file {
        dotenvy::from_path(&config_file).ok();
    }
}

#[main]
async fn main() {
    load_config_file();

    // Parse the CLI, ::try_parse() will return an error if the arguments are invalid or if `--help` is passed
    let cli = Cli::try_parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(ERR_PARSING_ARGS);
    });

    let config = ClientConfig::new(cli.host, cli.port);
    let mut client = build_client(config).unwrap_or_else(|e| {
        eprintln!("Error creating MQTT client: {}", e);
        exit(ERR_CREATING_MQTT_CLIENT);
    });

    // Connect to the MQTT server
    connect_client(&client).await.unwrap_or_else(|e| {
        eprintln!("Error connecting to MQTT server: {}", e);
        exit(ERR_CONNECTING_MQTT_BROKER);
    });

    // Get the topic and QoS from the CLI
    let topic = cli.topic;
    let qos: i32 = cli.qos as i32;

    // Execute the command based on the subcommand provided
    match cli.command {
        Commands::Pub { message } => publish(&client, topic, &message, qos).await,
        Commands::Sub {} => subscribe(&mut client, topic, qos).await,
    }
    .unwrap_or_else(|cmd_err| {
        eprintln!("{}", cmd_err);
        exit(ERR_FAILED_COMMAND);
    });
}
