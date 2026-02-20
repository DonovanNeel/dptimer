use clap::{Parser, Subcommand};
use tokio::process::Command;
use std::process::Stdio;
use std::env;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Starts the timer from a specific point in time, default = 0
    Start {
        #[arg(short, long, default_value_t = 0)]
        from: i32,
        #[arg(short, long, default_value_t = 's')]
        in_units: char,
        #[arg(short, long, default_value_t = false)]
        get_from_save: bool,
    },
    /// Pauses the timer
    Pause {

    },
    /// Resumes the timer from the pause
    Resume {

    },
    ///Reads the timer at a given point
    Read {

    },
    ///Adds to the timer
    Add {
        #[arg(short, long)]
        amount: i32,
    },
    /// Takes away from the timer
    Subtract {
        #[arg(short, long)]
        amount: i32,
    },
    ///Ends the timer
    End {

    },
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match env::current_dir() {
        Ok(path) => {
            println!("The current directory is {}", path.display());
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            std::process::exit(1);
        }
    }
    let mut path = env::current_dir().unwrap();
    path.push("timer");

    let mut timer_process = Command::new("cargo").args(["run"])
        .current_dir(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().expect("failed to spawn");

    match &cli.command {
        Commands::Start {from, in_units, get_from_save} => {
            let mut child_in = timer_process.stdin.take().unwrap();

            let _write_handle = child_in.write_all("start".as_bytes()).await;
            let _flush_handle = child_in.flush().await;
        }
        Commands::Pause {} => {}
        Commands::Resume {} => {}
        Commands::Read {} => {

        }
        Commands::Add {amount} => {

        }
        Commands::Subtract {amount} => {

        }
        Commands::End {} => {}
    }

    std::thread::sleep(std::time::Duration::from_secs(10));

}