mod time_converter;
mod selector;

use std::{
    fs::{
        OpenOptions,
        File
    },
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
use std::thread::current;
use std::time::Duration;
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
    let pause_value_false = 0;
    let pause_value_true = 1;

    match &cli.command {
        Commands::Init {} => {
            let file_result = OpenOptions::new()
                .write(true)
                .create(true)
                .open(timer_file_name);

            let _file = match file_result {
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
        Commands::Start { from, in_units} => {
            let file_result = OpenOptions::new()
                .append(true)
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

            let check_empty = time_selector.select_time('t');

            let is_empty: bool = match check_empty {
                Some(value) => false,
                None => true,
            };

            if !is_empty {
                return;
            }

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

            let writable_pause_state = format!("p: {}\n", pause_value_false);
            file.write(writable_pause_state.as_bytes()).unwrap();
        }
        Commands::Pause {} => {
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

            let total_time;

            if let Some(total_time_option) = time_selector.select_time('t') {
                total_time = total_time_option;
            }
            else {
                println!("Total time select not found!");
                return;
            }

            file.set_len(0).unwrap();
            file.rewind().unwrap();

            let writable_time_spent = format!("t: {}\n", total_time);

            file.write(writable_time_spent.as_bytes()).unwrap();

            let writable_start_time = format!("s: {}\n", 0);
            file.write(writable_start_time.as_bytes()).unwrap();

            let writable_pause_state = format!("p: {}\n", 1);
            file.write(writable_pause_state.as_bytes()).unwrap();
        }
        Commands::Resume {} => {
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

            let is_paused = match time_selector.select_time('p') {
                Some(p) => p == 1, //pause_value_true
                None => false,
            };

            if !is_paused {
                return;
            }

            let total_time;

            if let Some(total_time_option) = time_selector.select_time('t') {
                total_time = total_time_option;
            }
            else {
                println!("Total time select not found!");
                return;
            }

            file.set_len(0).unwrap();
            file.rewind().unwrap();

            let from_time = format!("t: {}\n", total_time);

            file.write(from_time.as_bytes()).unwrap();

            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();

            let writable_start_time = format!("s: {}\n", start_time.as_secs().to_string());
            file.write(writable_start_time.as_bytes()).unwrap();

            let writable_pause_state = format!("p: {}\n", pause_value_false);
            file.write(writable_pause_state.as_bytes()).unwrap();
        }

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

            let could_not_find_time_error_response = | | {
                println!("Could not find time spent!");
                -1
            };

            let time_spent = cycle_timer_and_get(file).unwrap_or_else(could_not_find_time_error_response);

            if time_spent == -1 { return }

            println!("Time spent: {}", from_i32_to_string(time_spent));
        }
        Commands::Add { amount } => {}
        Commands::Subtract { amount } => {}
        Commands::End {} => {}
    }
}

fn cycle_timer_and_get(mut file: File) -> Option<i32> {
    let reader = BufReader::new(&file);
    let mut time_selector = Selector::new(reader);

    let is_paused = match time_selector.select_time('p') {
        Some(p) => p == 1, //pause_value_true
        None => false,
    };

    let time_spent;
    let start_time = time_selector.select_time('s')?; // s for start
    let total_time = time_selector.select_time('t')?; // t for total

    let new_start_time;

    if is_paused {
        time_spent = total_time;
        new_start_time = Duration::new(0, 0);
    }
    else {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;

        println!("c{} s{} t{}", current_time, start_time, total_time); //TODO: remove test code

        let time_dif = current_time.checked_sub(start_time)?;
        time_spent = total_time.checked_add(time_dif)?;

        println!("Time spent: {}", from_i32_to_string(time_spent)); //TODO: remove this line

        new_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
    }

    //update text file
    file.set_len(0).unwrap();
    file.rewind().unwrap();

    let writable_time_spent = format!("t: {}\n", time_spent);

    file.write(writable_time_spent.as_bytes()).unwrap();

    let writable_start_time = format!("s: {}\n", new_start_time.as_secs().to_string());
    file.write(writable_start_time.as_bytes()).unwrap();

    let writable_pause_state = format!("p: {}\n", is_paused as i32);
    file.write(writable_pause_state.as_bytes()).unwrap();

    Some(time_spent)
}