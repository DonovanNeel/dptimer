pub fn from_i32_to_string(total_seconds: i32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn from_string_to_i32(total_time: &String) -> i32 {
    let segments = total_time.split(":").collect::<Vec<&str>>();

    let hours = segments[0].parse::<i32>().unwrap();
    let minutes = segments[1].parse::<i32>().unwrap();
    let seconds = segments[2].parse::<i32>().unwrap();
    hours * 3600 + minutes * 60 + seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_boundary() {
        // Zero is the lowest logical boundary for time
        let total = 0;
        let s = from_i32_to_string(total);
        assert_eq!(s, "00:00:00");
        assert_eq!(from_string_to_i32(&s), total);
    }

    #[test]
    fn test_typical_value() {
        // A standard value within bounds
        let total = 3661; // 1 hour, 1 min, 1 sec
        let s = from_i32_to_string(total);
        assert_eq!(s, "01:01:01");
        assert_eq!(from_string_to_i32(&s), total);
    }

    #[test]
    fn test_maximum_i32_boundary() {
        // The absolute limit of i32 (approx 68 years)
        let total = i32::MAX; // 2147483647
        let s = from_i32_to_string(total);
        println!("{:?}, {}", s, total);
        assert_eq!(s, "596523:14:07");
        assert_eq!(from_string_to_i32(&s), total);
    }

    #[test]
    fn test_negative_values() {
        // i32 allows negatives; ensures symmetry even if illogical for time
        let total = -3661;
        let s = from_i32_to_string(total);
        // Note: format! will result in "-01:-01:-01"
        assert_eq!(from_string_to_i32(&s), total);
    }

    #[test]
    fn test_large_hour_format() {
        // Test that it handles > 2 digits for hours (no truncation)
        let total = 360000; // 100 hours
        let s = from_i32_to_string(total);
        assert_eq!(s, "100:00:00");
    }
}