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
    CEST,  // Cetral European Summer Time
    MYT,   // Malaysia Time
    AWST,  // Australian Western Standard Time
    ACWST, // Australian Central Western Standard Time
    JST,   // Japan Standard Time
    ACST,  // Australian Central Standard Time
    AEST,  // Australian Eastern Standard Time
    LHST,  // Lord Howe Standard Time

    // negative offset
    EDT, // Eastern Daylight Time
    EST, // Eastern Standard Time
    PDT, // Pacific Daylight Time
}

impl Timezone {
    pub fn offset(&self) -> f32 {
        match self {
            Self::PDT => -7.0,
            Self::EST => -5.0,
            Self::EDT => -4.0,
            Self::UTC => 0.0,
            Self::CEST => 2.0,
            Self::MYT => 8.0,
            Self::AWST => 8.0,
            Self::ACWST => 8.75,
            Self::JST => 9.0,
            Self::ACST => 9.5,
            Self::AEST => 10.0,
            Self::LHST => 10.5,
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
            Self::AWST => String::from("AWST"),
            Self::ACWST => String::from("ACWST"),
            Self::JST => String::from("JST"),
            Self::ACST => String::from("ACST"),
            Self::AEST => String::from("AEST"),
            Self::LHST => String::from("LHST"),
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
            "AWST" => Ok(Timezone::AWST),
            "ACWST" => Ok(Timezone::ACWST),
            "JST" => Ok(Timezone::JST),
            "ACST" => Ok(Timezone::ACST),
            "AEST" => Ok(Timezone::AEST),
            "LHST" => Ok(Timezone::LHST),
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

    // TODO: Decouple to function convert_hour
    let hh_orig: f32 = caps
        .get(1)
        .map_or(0.0, |m| m.as_str().parse::<f32>().unwrap());
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
    let hh_converted: f32 = (hh_orig - source.offset() + target.offset()) % 24.0;
    let mm_offset: f32 = hh_converted % 1.0;
    let mut hh_new = if hh_converted.is_sign_negative() {
        24.0 + hh_converted - mm_offset
    } else {
        hh_converted - mm_offset
    };

    // TODO: Decouple to function convert_minute
    let mm_orig = caps
        .get(2)
        .map_or(0.0, |m| m.as_str().parse::<f32>().unwrap());
    let mm_converted: f32 = mm_offset.abs() * 60.0 + mm_orig;
    let mm_new = if mm_converted >= 60.0 {
        hh_new = hh_new + 1.0;
        mm_converted - 60.0
    } else {
        mm_converted
    };

    let ss = caps.get(3).map_or("", |m| m.as_str());

    let hh_string = if hh_new < 10.0 {
        "0".to_owned() + hh_new.to_string().as_str()
    } else {
        hh_new.to_string()
    };

    let mm_string = if mm_new < 10.0 {
        "0".to_owned() + mm_new.to_string().as_str()
    } else {
        mm_new.to_string()
    };

    hh_string + ":" + &mm_string + ":" + ss
}
