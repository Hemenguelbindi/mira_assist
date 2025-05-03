use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author="Hemenguelbindi", version="0.0.1",)]
pub struct StartMetricCli{
    #[command(subcommand)]
    pub commands: Command,
}


#[derive(Subcommand)]
pub enum Command{
    Start,
    Pause,
    Stop,
}
