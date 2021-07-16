use crate::data::HtRecord;
use crate::settings::Settings;
use chrono::{Duration, Utc, Date};
use std::ops::Add;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Summary {
    pub worked_time: Duration,
    pub expected_time: Duration,
    pub saldo: Duration,
    pub timesheet: Vec<PerDay>
}

#[derive(Debug)]
pub struct PerDay {
    pub date: Date<Utc>,
    pub worked_time: Duration,
    pub expected_time: Duration,
}


pub fn analyze_data(data: Vec<HtRecord>, settings: Settings) -> Summary {

    let mut expected_time = Duration::zero();
    let mut worked_time = Duration::zero();
    let timesheet = calc_timesheet(&data, settings).unwrap_or(Vec::new());

    for item in &timesheet {
        expected_time = expected_time.add(item.expected_time);
        worked_time = worked_time.add(item.worked_time);
    }

    Summary {
        worked_time,
        expected_time,
        saldo: worked_time - expected_time,
        timesheet
    }
}

/// Calculate the timesheet for the given input data.
///
/// The resulting vector will have exactly one entry for each day within the range determined
/// by the earliest and latest day in the input data.
/// Entries for days where the work time is less then our limit will have their expected work time
/// be set to 0.
fn calc_timesheet(data: &Vec<HtRecord>, settings: Settings) -> Option<Vec<PerDay>> {
    if data.is_empty() {
        return None;
    }

    let mut by_date: HashMap<Date<Utc>, Duration> = HashMap::with_capacity(data.len());
    for record in data {
        by_date
            .entry(record.clocked_in.date())
            .and_modify(|dur| *dur = dur.add(record.duration))
            .or_insert(record.duration);
    }

    let first_day = data.iter()
        .map(|data| data.clocked_in.date()).min().unwrap();
    let last_day = data.iter()
        .map(|data| data.clocked_in.date()).max().unwrap();

    let min_work_time = Duration::hours(settings.ignore_worked_hours_below as i64);
    let expected_time = Duration::hours(settings.working_hours as i64);
    let default_duration = Duration::zero();

    let mut for_all_days: Vec<PerDay> = Vec::with_capacity(by_date.len());

    let mut date = first_day;
    while date <= last_day {
        let worked_time = by_date.get(&date).unwrap_or(&default_duration);
        let expected_time = if worked_time < &min_work_time {
            Duration::zero()
        } else {
            expected_time
        };
        for_all_days.push(PerDay {
            date,
            worked_time: worked_time.clone(),
            expected_time
        });
        date = date.add(Duration::days(1));
    }

    return Some(for_all_days);
}
