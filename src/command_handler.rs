use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, Write, Error, ErrorKind};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::selector::Selector;
use crate::time_converter::{from_i32_to_string, from_string_to_i32};

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
    amount: String,
}
impl SubtractHandler {
    pub fn new(file_name: &str, amount: String) -> SubtractHandler {
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
    fn execute_command(self) -> Result<(),Error>;
}

impl Handler for InitHandler{
    fn execute_command(self) -> Result<(), Error> {
        let handle_result = get_file_handle(
            self.file_name,
            false,
            true,
            false,
            true
        );

        let return_value = match handle_result {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        };

        return_value
    }
}

impl Handler for StartHandler{
    fn execute_command(self) -> Result<(),Error> {
        let file = get_file_handle(
            self.file_name,
            true,
            true,
            true,
            false)?;

        let reader = BufReader::new(&file);
        let time_selector = Selector::new(reader);

        let check_or_total = get_total_and_check(time_selector);

        if check_or_total.is_ok() {
            return Err(Error::new(ErrorKind::Other, "Time is not empty"));
        }

        let mut from_value = self.from;

        let check_rollover = match self.in_units {
            'm' => 60,
            'h' => 3600,
            _ => 1
        };

        match from_value.checked_mul(check_rollover) {
            Some(value) => from_value = value,
            None => {
                println!("Value too large!\nInteger roled over.");
                return Err(Error::new(ErrorKind::Other, "Rollover Error"));
            }
        }

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        let writable_start_time = start_time.as_secs() as i32;

        set_file_content(file, from_value, writable_start_time, 0)
    }
}

impl Handler for PauseHandler{
    fn execute_command(self)  -> Result<(), Error> {

        let mut file = get_file_handle(
            self.file_name,
            true,
            true,
            false,
            false
        )?;

        let reader = BufReader::new(&file);
        let time_selector = Selector::new(reader);

        let total_time = get_total_and_check(time_selector)?;

        file.set_len(0)?;
        file.rewind()?;

        set_file_content(file, total_time, 0, 1)
    }
}

impl Handler for ResumeHandler{
    fn execute_command(self)  -> Result<(),Error> {
        let mut file = get_file_handle(
            self.file_name,
            true,
            true,
            false,
            false
        )?;

        let reader = BufReader::new(&file);
        let mut time_selector = Selector::new(reader);

        let is_paused = match time_selector.select_time('p') {
            Some(p) => p == 1, //pause_value_true
            None => false,
        };

        if !is_paused {
            return Ok(());
        }

        let total_time = get_total_and_check(time_selector)?;

        file.set_len(0)?;
        file.rewind()?;

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        let writable_start_time = start_time.as_secs() as i32;

        set_file_content(file, total_time, writable_start_time, 0)
    }
}

impl Handler for ReadHandler{
    fn execute_command(self)  -> Result<(),Error> {
        let file = get_file_handle(
            self.file_name,
            true,
            true,
            false,
            false
        )?;

        let could_not_find_time_error_response = | | {
            println!("Could not find time spent!");
            -1
        };

        let time_spent = cycle_timer_and_get(file).unwrap_or_else(could_not_find_time_error_response);

        if time_spent == -1 { return Err(Error::new(ErrorKind::Other, "Cycling time failed"));}

        println!("Time spent: {}", from_i32_to_string(time_spent)?);

        Ok(())
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

        let time_dif = current_time.checked_sub(start_time)?;
        time_spent = total_time.checked_add(time_dif)?;

        new_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
    }

    //update text file
    file.set_len(0).unwrap();
    file.rewind().unwrap();

    let writable_new_start_time = new_start_time.as_secs() as i32;
    let writable_pause_value = is_paused as i32;

    let set_result = set_file_content(
        file,
        time_spent,
        writable_new_start_time,
        writable_pause_value
    );

    let return_value = match set_result {
        Ok(_) => Some(time_spent),
        Err(_) => None
    };

    return_value
}

impl Handler for AddHandler{
    fn execute_command(self)  -> Result<(), Error> {
        modify_total_time(self.file_name, &self.amount, true)
    }
}

impl Handler for SubtractHandler {
    fn execute_command(self)  -> Result<(), Error> {
        modify_total_time(self.file_name, &self.amount, false)
    }
}

fn get_file_handle(file_name: String, read: bool, write: bool, append: bool, create: bool) -> Result<File, std::io::Error> {
    let file_result = OpenOptions::new()
        .read(read)
        .write(write)
        .append(append)
        .create(create)
        .open(file_name);

    let file = match file_result {
        Ok(file) => {
            Ok(file)
        }
        Err(err) => {
            println!("Get failed!");
            Err(err)
        }
    };

    file
}

fn set_file_content(
    file: File,
    total: i32,
    start: i32,
    pause: i32) -> Result<(), std::io::Error> {

    write_to_file_part(&file, 't', total)?;
    write_to_file_part(&file, 's', start)?;
    write_to_file_part(&file, 'p', pause)?;

    Ok(())
}

fn write_to_file_part(mut file: &File, part: char, time: i32) -> Result<(), std::io::Error> {
    let writable_time_spent = format!("{}: {}\n", part, time);
    file.write(writable_time_spent.as_bytes())?;

    Ok(())
}

fn get_total_and_check(mut time_selector: Selector) -> Result<i32, std::io::Error> {
    let total_time;

    if let Some(total_time_option) = time_selector.select_time('t') {
        total_time = total_time_option;
    }
    else {
        return Err(Error::new(ErrorKind::Other, "Time not found"));
    }
    Ok(total_time)
}

fn modify_total_time(file_name: String, amount_reference: &String, is_addition: bool) -> Result<(),Error>{
    let mut file = get_file_handle(
        file_name,
        true,
        true,
        false,
        false
    )?;

    let reader = BufReader::new(&file);
    let mut time_selector = Selector::new(reader);

    let start_time = time_selector.select_time('s');
    let pause_state = time_selector.select_time('p');
    let current_total_time = get_total_and_check(time_selector)?;

    let amount = from_string_to_i32(amount_reference)?;

    let matchable_option= match is_addition {
        true => current_total_time.checked_add(amount),
        false => current_total_time.checked_sub(amount)
    };

    let writable_total_time = match matchable_option {
        Some(total_time) => total_time,
        None => return Err(Error::new(ErrorKind::Other, "Time overflow"))
    };

    if writable_total_time < 0 {
        return Err(Error::new(ErrorKind::Other, "Time cannot be negative"));
    }

    file.set_len(0)?;
    file.rewind()?;

    set_file_content(file, writable_total_time, start_time.unwrap(), pause_state.unwrap())
}