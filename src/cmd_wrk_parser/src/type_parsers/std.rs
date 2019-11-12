use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use lazy_static::lazy_static;

use crate::{TypeParser, Error, TypeParseResult, utils};
use regex::Regex;

lazy_static! {
    static ref DURATION_FMT_1: Regex = Regex::new(
        r"((?P<Hour>\d{1,4}):)?(?P<Minute>\d{1,2}):(?P<Second>\d{1,2})(\.(?P<Milli>\d{1,3}))?"
    ).unwrap();

    static ref DURATION_FMT_2: Regex = Regex::new(r"(\d{1,4})(y|M|w|d|h|m|s|ms|ns)").unwrap();
}

pub struct IpParser;

impl TypeParser for IpParser {
    type Type = IpAddr;
    type Context = ();

    fn parse(&self, _ctx: &Self::Context, src: &str) -> TypeParseResult<Self::Type> {
        let s = match utils::take_until_whitespace(src) {
            Some(s) => s,
            None => return Err(Error::UnexpectedToken(0)),
        };

        let val = IpAddr::from_str(s)?;

        Ok((val, s.len()))
    }
}

pub struct DurationParser;

impl TypeParser for DurationParser {
    type Type = Duration;
    type Context = ();

    fn parse(&self, _ctx: &Self::Context, src: &str) -> TypeParseResult<Self::Type> {
        let mut out = Duration::new(0, 0);

        if let Some(c) = DURATION_FMT_1.captures(src) {
            if let Some(hour) = c.name("Hour") {
                out += Duration::from_secs(u64::from_str(hour.as_str())? * 3600);
            }

            if let Some(min) = c.name("Minute") {
                out += Duration::from_secs(u64::from_str(min.as_str())? * 60);
            }

            if let Some(sec) = c.name("Second") {
                out += Duration::from_secs(u64::from_str(sec.as_str())?);
            }

            if let Some(ms) = c.name("Milli") {
                out += Duration::from_millis(u64::from_str(ms.as_str())?);
            }

            let len = c[0].len();

            return Ok((out, len))
        }

        let mut counter = 0u8;
        let mut end: usize = 0;

        for (i, c) in DURATION_FMT_2.captures_iter(src).enumerate() {
            if counter == 0 && i == 0 {
                continue
            } else { counter += 1 }

            let mut val = u64::from_str(c.get(1).unwrap().as_str())?;

            match c.get(2).unwrap().as_str() {
                "y"  => val *= 1_000_000 * 1_000 * 60 * 60 * 24 * 30 * 12,
                "M"  => val *= 1_000_000 * 1_000 * 60 * 60 * 24 * 30,
                "w"  => val *= 1_000_000 * 1_000 * 60 * 60 * 24 * 7,
                "d"  => val *= 1_000_000 * 1_000 * 60 * 60 * 24,
                "h"  => val *= 1_000_000 * 1_000 * 60 * 60,
                "m"  => val *= 1_000_000 * 1_000 * 60,
                "s"  => val *= 1_000_000 * 1_000,
                "ms" => val *= 1_000_000,
                "ns" => (),
                _ => unreachable!(),
            }

            out += Duration::from_nanos(val);

            end = c[0].len();
        }

        if counter > 0 {
            Ok((out, end))
        } else {
            Err(Error::InvalidFormat(String::from("unable to process duration")))
        }
    }
}

pub struct StringParser;

impl TypeParser for StringParser {
    type Type = String;
    type Context = ();

    fn parse(&self, _ctx: &Self::Context, src: &str) -> TypeParseResult<Self::Type> where
    {
        let s = if let Some(s) = utils::take_until_close_quote(src) { s }
            else { utils::take_until_whitespace(src).unwrap() };

        Ok((String::from(s), s.len()))
    }
}
