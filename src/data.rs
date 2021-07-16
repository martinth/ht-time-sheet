use csv::{ReaderBuilder, StringRecord};
use std::path::Path;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Duration, NaiveDateTime};
use anyhow::anyhow;

#[derive(Debug)]
pub struct HtRecord {
    pub job: String,
    pub clocked_in: DateTime<Utc>,
    pub clocked_out: DateTime<Utc>,
    pub duration: Duration
}

pub fn read_data(input: &Path) -> Result<Vec<HtRecord>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input)?;

    let mut data: Vec<HtRecord> = Vec::with_capacity(10);

    for result in reader.records() {
        let record = result?;
        let parsed = parse_record(&record)?;
        data.push(parsed);
    }

    Ok(data)
}


fn parse_record(record: &StringRecord) -> Result<HtRecord> {
    // record is: "Job","Clocked In","Clocked Out","Duration","Comment","Breaks","Adjustments","Mileage"
    let job = &record[0];
    let raw_in = &record[1];
    let raw_out = &record[2];
    let raw_duration = &record[3];

    let clocked_in = NaiveDateTime::parse_from_str(raw_in, "%d.%m.%Y %H:%M")
        .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc))
        .with_context(|| format!("clocked in time not parseable: {:?}", raw_in))?;
    let clocked_out = NaiveDateTime::parse_from_str(raw_out, "%d.%m.%Y %H:%M")
        .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc))
        .with_context(|| format!("clocked out time not parseable: {:?}", raw_in))?;
    let duration_parts: Vec<&str> = raw_duration.split(":").collect();
    if duration_parts.len() != 2 {
        return Err(anyhow!("Could not parse duration out of: {:?}", raw_duration));
    }

    let duration_hours: i64 =  duration_parts.get(0).unwrap().parse()?;
    let duration_minutes: i64 =  duration_parts.get(1).unwrap().parse()?;
    let duration = Duration::minutes(duration_minutes + (duration_hours * 60));

    Ok(HtRecord{
        job: String::from(job),
        clocked_in,
        clocked_out,
        duration
    })

}
