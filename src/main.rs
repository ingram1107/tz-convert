use clap::Parser;
use regex::Regex;
use std::str::FromStr;

mod tz;
use tz::*;

#[derive(Parser)]
#[clap(author, version, arg_required_else_help(true))]
/// A simple timezone converter
struct Args {
    #[clap(short, long, value_name = "TIME", validator = valid_time_format)]
    /// Time to be converted, in the format HH:MM:SS
    time: Option<String>,

    #[clap(short = 'S', long, value_name = "TIMEZONE", validator = valid_timezone)]
    /// Source timezone to be converted, default as UTC
    source: Option<String>,

    #[clap(short = 'T', long, value_name = "TIMEZONE", validator = valid_timezone)]
    /// Target timezone to convert to, default as UTC
    target: Option<String>,
}

fn main() {
    let args = Args::parse();

    let time = match args.time {
        Some(time) => time,
        _ => String::new(),
    };

    let source_tz = match args.source {
        Some(tz) => match Timezone::from_str(tz.as_str()) {
            Ok(tz) => tz,
            _ => Timezone::UTC,
        },
        _ => Timezone::UTC,
    };

    let target_tz = match args.target {
        Some(tz) => match Timezone::from_str(tz.as_str()) {
            Ok(tz) => tz,
            _ => Timezone::UTC,
        },
        _ => Timezone::UTC,
    };

    let time_str = time.as_str();
    let source_tz_string = source_tz.to_string();
    let target_tz_string = target_tz.to_string();
    let time_new = convert_time(time_str, source_tz, target_tz);

    println!(
        "{} {} -> {} {}",
        source_tz_string, time_str, target_tz_string, time_new
    );
}

fn valid_time_format(s: &str) -> Result<(), String> {
    let re = Regex::new(r"([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]").unwrap();

    //TODO better error matching to know that particular field should not exceed the value limit
    //eg:
    //  hh < 24
    //  mm < 60
    //  ss < 60
    match re.is_match(s) {
        true => Ok(()),
        false => Err(format!("Doesn't match the format hh:mm:ss in 24h")),
    }
}

fn valid_timezone(s: &str) -> Result<(), String> {
    match Timezone::from_str(s) {
        Err(error) => Err(format!("{}", error)),
        _ => Ok(()),
    }
}
