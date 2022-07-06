use regex::Regex;
use std::fmt;
use std::str::FromStr;

pub struct TimezoneNotSupportedErr {
    timezone: String,
}

impl fmt::Display for TimezoneNotSupportedErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timezone {} not supported!", self.timezone)
    }
}

impl fmt::Debug for TimezoneNotSupportedErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TimezoneNotSupportedErr {{ timezone: {} }}",
            self.timezone
        )
    }
}

pub enum Timezone {
    UTC, // Coordinated Universal Time, also GTM

    // positive offset
    CEST, // Cetral European Summer Time
    MYT,  // Malaysia Time
    JST,  // Japan Standard Time

    // negative offset
    EDT, // Eastern Daylight Time
    EST, // Eastern Standard Time
    PDT, // Pacific Daylight Time
}

impl Timezone {
    pub fn offset(&self) -> i8 {
        match self {
            Self::PDT => -7,
            Self::EST => -5,
            Self::EDT => -4,
            Self::UTC => 0,
            Self::CEST => 2,
            Self::MYT => 8,
            Self::JST => 9,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::PDT => String::from("PDT"),
            Self::EST => String::from("EST"),
            Self::EDT => String::from("EDT"),
            Self::UTC => String::from("UTC"),
            Self::CEST => String::from("CEST"),
            Self::MYT => String::from("MYT"),
            Self::JST => String::from("JST"),
        }
    }
}

impl FromStr for Timezone {
    type Err = TimezoneNotSupportedErr;

    fn from_str(input: &str) -> Result<Timezone, Self::Err> {
        match input {
            "UTC" | "GMT" => Ok(Timezone::UTC),
            "CEST" => Ok(Timezone::CEST),
            "MYT" => Ok(Timezone::MYT),
            "JST" => Ok(Timezone::JST),
            "EDT" => Ok(Timezone::EDT),
            "EST" => Ok(Timezone::EST),
            "PDT" => Ok(Timezone::PDT),
            _ => Err(TimezoneNotSupportedErr {
                timezone: input.to_string(),
            }),
        }
    }
}

pub fn convert_time(time_orig: &str, source: Timezone, target: Timezone) -> String {
    let re = Regex::new(r"([0-1][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9])").unwrap();
    let caps = re.captures(time_orig).unwrap();

    let hh_orig: i8 = caps.get(1).map_or(0, |m| m.as_str().parse::<i8>().unwrap());
    // Equation:
    //      time in zone A - (UTC offset for zone A) + (UTC offset for zone B) = time in zone B
    //
    // Cases:
    //      1. The result could be more than 24h
    //      2. The result could be negative
    //
    // Solutions:
    //      1. Mod the result from equation
    //      2. Use equation 24 + hh to normalise the negative result
    let hh_converted: i8 = (hh_orig - source.offset() + target.offset()) % 24;
    let hh_new = if hh_converted.is_negative() {
        24 + hh_converted
    } else {
        hh_converted
    };

    //TODO might need to change
    let mm = caps.get(2).map_or("", |m| m.as_str());
    let ss = caps.get(3).map_or("", |m| m.as_str());

    let hh_string = if hh_new < 10 {
        "0".to_owned() + hh_new.to_string().as_str()
    } else {
        hh_new.to_string()
    };

    hh_string + ":" + mm + ":" + ss
}
