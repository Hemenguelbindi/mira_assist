mod command;
mod manager;

use clap::Parser;
use command::StartMetricCli;
use manager::MiraMenagerCLI;

fn main() {
    let cli =  StartMetricCli::parse();
    let mut manager = MiraMenagerCLI::new();

    manager.execute(cli.commands);
}
