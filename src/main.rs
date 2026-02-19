use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Sets up the timer
    Init {

    },
    /// Starts the timer from a specific point in time, default = 0
    Start {
        #[arg(short, long, default_value_t = 0)]
        from: i32,
        #[arg(short, long, default_value_t = 's')]
        in_units: char
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
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => {}
        Commands::Start {from, in_units} => {

        }
        Commands::Pause {} => {}
        Commands::Resume {} => {}
        Commands::Read {} => {}
        Commands::Add {amount} => {

        }
        Commands::Subtract {amount} => {

        }
    }
}
//
