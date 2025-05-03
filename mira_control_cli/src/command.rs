use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author="Hemenguelbindi", version="0.0.1",)]
pub struct StartMetric{
    #[command(subcommand)]
    pub commands: Commansd,
}


pub enum Commansd{
    Start,
    Pause,
    Stop,
}
