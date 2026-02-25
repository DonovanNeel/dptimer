mod time_converter;

use std::{
    fs::OpenOptions,
    time::{
        SystemTime,
        UNIX_EPOCH
    },
    io::{
        self,
        Write,
        BufRead,
        BufReader,
    },
};
use std::io::Seek;
use clap::{Parser, Subcommand};
use crate::time_converter::from_i32_to_string;

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
                Ok(file) =>  {
                    println!("Timer initialized successfully!");
                    file
                }
                Err(_) => {
                    println!("Unable initialize!");
                    return;
                }
            };

            file.write(b"t: 0\n").unwrap();

        }
        Commands::Start {from, in_units, get_from_save} => {

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

            let file = match file_result {
                Ok(file) => file,
                Err(_) => {
                    println!("Could not find timer!");
                    return;
                }
            };

            let mut reader = BufReader::new(&file);

            //get current time
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();

            //use current time and start time to find the time spent
            let mut buffer = Vec::new();
            reader.read_until(b's', &mut buffer).unwrap();
            buffer.clear();
            reader.read_until(b'\n', &mut buffer).unwrap();

            let temp_start_time = String::from_utf8_lossy(&buffer);

            let start_time_unclipped = temp_start_time
                .split(' ')
                .collect::<Vec<&str>>()[1];

            let start_time = start_time_unclipped.trim_end().parse::<i32>().unwrap();

            let time_spent =  current_time.as_secs() as i32 - start_time;

            //update the total time spent (marked with "t: " in the text file)

            //set the start time to the current time (marked with "s: " in the text file)



        }
        Commands::Add {amount} => {

        }
        Commands::Subtract {amount} => {

        }
        Commands::End {} => {}
    }

}