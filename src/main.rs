mod time_converter;
mod selector;
mod command_handler;

use clap::{Parser, Subcommand};
use crate::command_handler::{
    Handler,
    InitHandler,
    PauseHandler,
    ReadHandler,
    ResumeHandler,
    StartHandler,
};

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
            InitHandler::new(timer_file_name).execute_command();
        }
        Commands::Start { from, in_units} => {

            StartHandler::new(timer_file_name, *from, *in_units).execute_command();
        }
        Commands::Pause {} => {
            PauseHandler::new(timer_file_name).execute_command();
        }
        Commands::Resume {} => {
            ResumeHandler::new(timer_file_name).execute_command();
        }
        Commands::Read {} => {
            ReadHandler::new(timer_file_name).execute_command();
        }
        Commands::Add { amount } => {}
        Commands::Subtract { amount } => {}
        Commands::End {} => {}
    }
}