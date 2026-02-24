mod time_converter;

use std::{
    fs::OpenOptions,
    time::{
        SystemTime,
        UNIX_EPOCH
    },
};
use std::io::Write;
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes the timer
    Init {

    },
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



fn main() {
    let cli = Cli::parse();
    let timer_file_name = "dptimer.txt";

    match &cli.command {
        Commands::Init {} => {
            let file_result = OpenOptions::new()
                .write(true)
                .create(true)
                .open(timer_file_name);
            match file_result {
                Ok(_) => {
                    println!("Timer initialized successfully!");
                }
                Err(_) => {
                    println!("Unable initialize!");
                }
            }
        }
        Commands::Start {from, in_units, get_from_save} => {

            let file_result = OpenOptions::new()
                .write(true)
                .open(timer_file_name);

            let mut file = match file_result {
                Ok(file) => file,
                Err(_) => {
                    println!("Could not find timer!");
                    return;
                }
            };

            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();

            let writable_start_time = start_time.as_secs().to_string();
            file.write(writable_start_time.as_bytes()).unwrap();

            println!("Start timer successfully! Start time: {}",
                 time_converter::from_i32_to_string(
                     writable_start_time
                         .parse::<i32>()
                         .unwrap()
                 )
            );
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

}