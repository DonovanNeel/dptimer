use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::selector::Selector;
use crate::time_converter::from_i32_to_string;

pub struct InitHandler {
    file_name: String,
}
impl InitHandler {
    pub fn new(file_name: &str) -> InitHandler {
        InitHandler{file_name: file_name.to_string()}
    }
}
pub struct StartHandler{
    file_name: String,
    from: i32,
    in_units: char,
}
impl StartHandler {
    pub fn new(file_name: &str, from: i32, in_units: char) -> StartHandler {
        StartHandler{file_name: file_name.to_string(), from, in_units }
    }
}
pub struct PauseHandler{
    file_name: String,
}
impl PauseHandler {
    pub fn new(file_name: &str) -> PauseHandler {
        PauseHandler{file_name: file_name.to_string()}
    }
}
pub struct ResumeHandler{
    file_name: String,
}
impl ResumeHandler {
    pub fn new(file_name: &str) -> ResumeHandler {
        ResumeHandler{file_name: file_name.to_string()}
    }
}
pub struct ReadHandler{
    file_name: String,
}
impl ReadHandler {
    pub fn new(file_name: &str) -> ReadHandler {
        ReadHandler{file_name: file_name.to_string()}
    }
}
pub struct AddHandler{
    file_name: String,
    amount: String
}
impl AddHandler {
    pub fn new(file_name: &str, amount: String) -> AddHandler {
        AddHandler{file_name: file_name.to_string(), amount }
    }
}
pub struct SubtractHandler {
    file_name: String,
    amount: i32
}
impl SubtractHandler {
    pub fn new(file_name: &str, amount: i32) -> SubtractHandler {
        SubtractHandler{file_name: file_name.to_string(), amount }
    }
}
pub struct EndHandler{
    file_name: String,
}
impl EndHandler {
    pub fn new(file_name: &str) -> EndHandler {
        EndHandler{file_name: file_name.to_string()}
    }
}

pub trait Handler{
    fn execute_command(self);
}

impl Handler for InitHandler{
    fn execute_command(self){
        let file_result = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.file_name);

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
}

impl Handler for StartHandler{
    fn execute_command(self){
        let file_result = OpenOptions::new()
            .append(true)
            .read(true)
            .write(true)
            .open(self.file_name);

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
            Some(_value) => false,
            None => true,
        };

        if !is_empty {
            return;
        }

        let mut from_value = self.from;

        if self.in_units == 'm' {
            match from_value.checked_mul(60) {
                Some(value) => from_value = value,
                None => {
                    println!("Value too large!\nInteger roled over.");
                    return;
                }
            }
        }
        if self.in_units == 'h' {
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

        let writable_pause_state = format!("p: {}\n", 0); // is for pause value false
        file.write(writable_pause_state.as_bytes()).unwrap();
    }
}

impl Handler for PauseHandler{
    fn execute_command(self){
        let file_result = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_name);

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

        let writable_pause_state = format!("p: {}\n", 1); // 1 for pause true
        file.write(writable_pause_state.as_bytes()).unwrap();
    }
}

impl Handler for ResumeHandler{
    fn execute_command(self){
        let file_result = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_name);

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

        let writable_pause_state = format!("p: {}\n", 0);
        file.write(writable_pause_state.as_bytes()).unwrap();
    }
}

impl Handler for ReadHandler{
    fn execute_command(self){
        let file_result = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_name);

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

impl Handler for AddHandler{
    fn execute_command(self){
        let file_result = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_name);

        let file = match file_result {
            Ok(file) => file,
            Err(_) => {
                println!("Could not find timer!");
                return;
            }
        };


    }
}