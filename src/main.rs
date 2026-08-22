use std::io::{self, Write};

use time::format_description::FormatItem;
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};

static FORMATS: &[&[FormatItem<'static>]] = &[
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second][offset_hour]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour]"),
    format_description!("[year]/[month]/[day] [hour]:[minute]:[second]"),
    format_description!("[year]/[month]/[day] [hour]:[minute]:[second].[subsecond]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour]:[offset_minute]"),
];

fn parse_any(s: &str) -> Option<OffsetDateTime> {
    for fmt in FORMATS {
        if let Ok(dt) = OffsetDateTime::parse(s, fmt) {
            return Some(dt);
        }
        if let Ok(dt) = PrimitiveDateTime::parse(s, fmt) {
            return Some(dt.assume_utc());
        }
    }
    None
}

fn epoch_millis(dt: OffsetDateTime) -> i128 {
    dt.unix_timestamp_nanos() / 1_000_000
}

fn main() {
    println!("Enter date-time string to get Epoch millis.");

    loop {
        let mut buf = String::new();
        print!("> ");

        io::stdout().flush().unwrap();

        let nread = io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");
        let input = buf.trim();

        if nread == 0 {
            println!();
            break;
        }

        if input.is_empty() {
            continue;
        }

        match parse_any(input) {
            Some(dt) => println!("  {}", epoch_millis(dt)),
            None => eprintln!("Unidentified date-time format."),
        }
    }
}
