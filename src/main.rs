mod time_converter;
mod selector;
mod command_handler;

use std::io::Error;
use clap::{Parser, Subcommand};
use crate::command_handler::{AddHandler, Handler, InitHandler, PauseHandler, ReadHandler, ResumeHandler, StartHandler, SubtractHandler};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes the timer
    Init { },
    /// Starts the timer from a specific point in time, default = 0
    Start {
        #[arg(short, long, default_value_t = 0)]
        from: i32,
        #[arg(short, long, default_value_t = 's')]
        in_units: char,
    },
    /// Pauses the timer
    Pause { },
    /// Resumes the timer from the pause
    Resume { },
    ///Reads the timer at a given point
    Read { },
    ///Adds to the timer
    Add {
        #[arg(short, long)]
        amount: String,
    },
    /// Takes away from the timer
    Subtract {
        #[arg(short, long)]
        amount: String,
    },
    ///Ends the timer
    End { },
}



fn main() {
    let cli = Cli::parse();
    let timer_file_name = "dptimer.txt";

    match &cli.command {
        Commands::Init {} => {
            let execute_result = InitHandler::new(timer_file_name).execute_command();
            check_result(execute_result);
        }
        Commands::Start { from, in_units} => {

            let execute_result = StartHandler::new(timer_file_name, *from, *in_units)
                .execute_command();
            check_result(execute_result);
        }
        Commands::Pause {} => {
            let execute_result = PauseHandler::new(timer_file_name).execute_command();
            check_result(execute_result);
        }
        Commands::Resume {} => {
            let execute_result = ResumeHandler::new(timer_file_name).execute_command();
            check_result(execute_result);
        }
        Commands::Read {} => {
            let execute_result = ReadHandler::new(timer_file_name).execute_command();
            check_result(execute_result);
        }
        Commands::Add { amount } => {
            let execute_result = AddHandler::new(timer_file_name, amount.clone()).execute_command();
            check_result(execute_result);
        }
        Commands::Subtract { amount } => {
            let execute_result = SubtractHandler::new(timer_file_name, amount.clone()).execute_command();
            check_result(execute_result);
        }
        Commands::End {} => {}
    }
}

fn check_result(result: Result<(), Error>) {
    match result {
        Ok(_) => { }
        Err(error) => {
            println!("Failed with error: {}", error);
        }
    }
}