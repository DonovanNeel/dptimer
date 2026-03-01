use std::io::{Error, ErrorKind};

pub fn from_i32_to_string(total_seconds: i32) -> Result<String, Error> {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    Ok(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
}

pub fn from_string_to_i32(total_time: &String) -> Result<i32, Error> {
    let segments = total_time.split(":").collect::<Vec<&str>>();

    let make_error = || Error::new(ErrorKind::InvalidData, "invalid format. Use \"xx:xx:xx\"");

    if segments.len() != 3 {
        return Err(make_error());
    }

    let hours = segments[0].parse::<i32>()
        .map_err(|_| make_error())?;
    let minutes = segments[1].parse::<i32>()
        .map_err(|_| make_error())?;
    let seconds = segments[2].parse::<i32>()
        .map_err(|_| make_error())?;

    Ok(hours * 3600 + minutes * 60 + seconds)
}