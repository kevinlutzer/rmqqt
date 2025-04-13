use clap::{Parser, Subcommand};

/// Simple MQTT CLI tool with TLS and auth support
#[derive(Parser)]
#[command(
    name = "mqttcli",
    version,
    author,
    about = "Send and subscribe to MQTT topics"
)]
pub struct Cli {
    #[arg(short, long)]
    pub host: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>,

    #[arg(long, short)]
    pub topic: String,

    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, short, default_value_t = 1)]
    pub qos: i32,
}

/// Commands for the MQTT CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Publish a message to the specified topics. The QOS used is the same for all messages published topics
    /// The message is sent as a utf-8 encoded string.
    Pub { message: String },

    /// Subscribe to a topic. This will continuously print messages received on the topics specified until
    /// the program is terminated or the connection to the server is lost. Received payloads are treated as utf-8 encoded strings.
    Sub {},
}
