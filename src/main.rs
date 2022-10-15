use std::{
    path::PathBuf,
    process::{exit, Command},
};

use clap::Parser;
use inotify::{Inotify, WatchMask};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    watch: PathBuf,

    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.command.len() == 0 {
        println!("Must specify at least one positional argument for the managed subprocess. Use --help for more details.");
        exit(1);
    }

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .add_watch(
            cli.watch,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];
    loop {
        let cli_command = cli.command.to_owned();

        let args = &cli_command[1..];
        let program = &cli_command.to_owned()[0];

        let mut cmd_base = Command::new(program);
        let mut cmd = cmd_base
            .args(args)
            .spawn()
            .expect("Could not spawn subprocess");

        println!("waiting for events...");
        inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        println!("restarting process...");
        cmd.kill().expect("!kill");
        cmd.wait().expect("process failed");
    }
}
