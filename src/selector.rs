use std::{
    io::{
        BufRead,
        BufReader},
    fs::File
};
use std::io::Seek;

pub struct Selector<'a> {
    reader: BufReader<&'a File>,
}

impl<'a> Selector<'a>{
    pub fn new(reader: BufReader<&'a File>) -> Self {
        Selector{reader}
    }

    pub fn select_time(&mut self, time_type: char) -> Option<i32> {
        let mut buffer = Vec::new();

        self.reader.rewind().unwrap();

        self.reader.read_until(time_type as u8, &mut buffer).unwrap();
        buffer.clear();
        self.reader.read_until(b'\n', &mut buffer).unwrap();
        if buffer.is_empty() {
            return None;
        }

        let temp_time = String::from_utf8_lossy(&buffer);

        let time_unclipped = temp_time
            .split(' ')
            .collect::<Vec<&str>>()[1];

        let time = time_unclipped
            .trim_end()
            .parse::<i32>()
            .unwrap();

        Some(time)
    }
}