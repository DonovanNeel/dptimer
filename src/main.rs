mod time_converter;
mod selector;

use std::{
    fs::OpenOptions,
    time::{
        SystemTime,
        UNIX_EPOCH
    },
    io::{
        Write,
        BufReader,
        Seek,
    },
};
use clap::{Parser, Subcommand};
use crate::time_converter::from_i32_to_string;
use crate::selector::Selector;

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

            let mut file = match file_result {
                Ok(file) => {
                    println!("Timer initialized successfully!");
                    file
                }
                Err(_) => {
                    println!("Unable initialize!");
                    return;
                }
            };
        }
        Commands::Start { from, in_units, get_from_save } => {
            let file_result = OpenOptions::new()
                .append(true)
                .write(true)
                .open(timer_file_name);

            let mut file = match file_result {
                Ok(file) => file,
                Err(_) => {
                    println!("Could not find timer!");
                    return;
                }
            };

            let mut from_value = *from;

            if *in_units == 'm' {
                match from_value.checked_mul(60) {
                    Some(value) => from_value = value,
                    None => {
                        println!("Value too large!\nInteger roled over.");
                        return;
                    }
                }
            }
            if *in_units == 'h' {
                match from_value.checked_mul(3600) {
                    Some(value) => from_value = value,
                    None => {
                        println!("Value too large!\nInteger roled over.");
                        return;
                    }
                }
            }

            let from_time = format!("t: {}\n", from_value);

            file.write(from_time.as_bytes()).unwrap();

            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();

            let writable_start_time = format!("s: {}\n", start_time.as_secs().to_string());
            file.write(writable_start_time.as_bytes()).unwrap();
        }
        Commands::Pause {} => {}
        Commands::Resume {} => {}

        Commands::Read {} => {
            let file_result = OpenOptions::new()
                .read(true)
                .write(true)
                .open(timer_file_name);

            let mut file = match file_result {
                Ok(file) => file,
                Err(_) => {
                    println!("Could not find timer!");
                    return;
                }
            };

            let reader = BufReader::new(&file);
            let mut time_selector = Selector::new(reader);

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32;

            let start_time = time_selector.select_time('s'); // s for start
            let total_time = time_selector.select_time('t'); // t for total

            println!("{} {}", start_time, total_time); //TODO: remove test code

            let time_spent = total_time + current_time - start_time;

            println!("Time spent: {}", from_i32_to_string(time_spent));

            //update text file
            file.set_len(0).unwrap();
            file.rewind().unwrap();

            let writable_time_spent = format!("t: {}\n", time_spent);

            file.write(writable_time_spent.as_bytes()).unwrap();

            let new_start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();

            let writable_start_time = format!("s: {}\n", new_start_time.as_secs().to_string());
            file.write(writable_start_time.as_bytes()).unwrap();
        }
        Commands::Add { amount } => {}
        Commands::Subtract { amount } => {}
        Commands::End {} => {}
    }
}