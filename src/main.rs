mod settings;
mod data;
mod timesheet;

use settings::Settings;
use std::env;
use std::path::Path;
use anyhow::Result;
use anyhow::anyhow;
use crate::timesheet::{analyze_data, Summary, PerDay};
use chrono::Datelike;
use hhmmss::Hhmmss;

fn main() {
    let settings = Settings::new().expect("Unable to load settings");
    let args: Vec<String> = env::args().collect();
    let data = args.get(1).ok_or(anyhow!("Expected input file a first argument"))
        .and_then(|filename| validate_file_readable(filename))
        .and_then(|path| data::read_data(path)).unwrap();

    let summary = analyze_data(data, settings);
    display_summary(summary);
}

fn display_summary(summary: Summary) {
    if summary.timesheet.is_empty() {
        println!("No timesheet data in input");
        return;
    }

    let mut last: Option<PerDay> = None;
    for entry in summary.timesheet {
        if entry.date.weekday().num_days_from_monday() < 5 {
            if last.is_none() || last.unwrap().date.iso_week().week() != entry.date.iso_week().week() {
                println!("\nWeek {}", entry.date.format("%Y-%U"));
            }
            println!("  {}: {}", entry.date.format("%a %Y-%m-%d"), entry.worked_time.hhmmss());
        }
        last = Some(entry);
    }

    println!();
    println!("Total time worked {}", summary.worked_time.hhmmss());
    println!("Total expected    {}", summary.expected_time.hhmmss());
    println!("Saldo             {}", summary.saldo.hhmmss());
}

fn validate_file_readable(val: &str) -> Result<&Path> {
    let path = Path::new(val);
    if path.exists() {
        Ok(path)
    } else {
        Err(anyhow!("{:?} does not exist"))
    }
}
