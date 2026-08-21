use std::io;
use std::io::Write;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

fn main() {
    println!("Enter date-time string to get Epoch millis.");
    let datetime_fmts = vec![
        "%Y-%m-%d %H:%M:%S"
    ];

    loop {
        let mut input = String::new();
        print!("> ");

        io::stdout().flush().unwrap();

        let nread = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();

        if nread == 0 {
            println!();
            break;
        }

        if input.is_empty() {
            continue;
        }

        if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S.%f") {
            let dt_utc = Utc.from_utc_datetime(&dt);
            let epoch = dt_utc.timestamp_millis();
            println!("{epoch}");
        } else {
            for fmt in &datetime_fmts {
                match DateTime::parse_from_str(input, fmt) {
                    Ok(dt) => println!("{}", dt.timestamp_millis()),
                    Err(e) => eprintln!("Skip this pattern: {fmt}\nerror: {e}"),
                }
            }
        }
    }
}
