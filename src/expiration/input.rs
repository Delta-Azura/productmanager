use chrono::NaiveDate;
use anyhow::{Context, Result};


pub fn input(code: &str, date: &str, qt: &str) -> Result<()> {
    let dateinputs: Vec<&str> = date.split("/").collect();
    if dateinputs.len() != 3 {
        anyhow::bail!("Date format invalid");
    }
    let day: u32 = dateinputs[0].parse().context("Failed to get the day")?;
    let month: u32 = dateinputs[1].parse().context("Failed to get the month")?;
    let year: i32 = dateinputs[2].parse().context("Failed to get the year")?;
    let date = NaiveDate::from_ymd_opt(year, month, day).context("Date format isn't valid")?;
    Ok(())
}




