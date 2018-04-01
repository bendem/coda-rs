extern crate chrono;

use std::ops::Range;
use chrono::NaiveDate;

use errors::*;

pub fn parse_date(s: &str) -> Result<NaiveDate> {
    let date: NaiveDate =
        NaiveDate::parse_from_str(s, "%d%m%y").chain_err(|| "Could not parse date")?;

    Ok(date)
}

pub fn parse_str(s: &str) -> Result<String> {
    Ok(String::from(s))
}

pub fn parse_u8(s: &str) -> Result<u8> {
    Ok(s.parse::<u8>().chain_err(|| "Could not parse u8")?)
}

pub fn parse_u64(s: &str) -> Result<u64> {
    Ok(s.parse::<u64>().chain_err(|| "Could not parse u64")?)
}

pub fn parse_duplicate(s: &str) -> Result<bool> {
    match s {
        "D" => Ok(true),
        " " => Ok(false),
        _ => Err(format!("Invalid duplicate value [{}]", s).into()),
    }
}

pub fn parse_field<T>(
    line: &str,
    range: Range<usize>,
    convert: fn(s: &str) -> Result<T>,
) -> Result<T> {
    if let Some(part) = line.get(range) {
        convert(part)
    } else {
        Err("Could not parse field".into())
    }
}

#[cfg(test)]
mod test_parse_utils {
    use chrono::NaiveDate;

    use super::parse_field;
    use super::parse_date;
    use super::parse_str;
    use super::parse_duplicate;
    use super::parse_u8;
    use super::parse_u64;

    #[test]
    fn parse_date_valid() {
        let actual = parse_date("290318");

        assert_eq!(actual.is_ok(), true, "Date should be ok");
        assert_eq!(
            actual.unwrap(),
            NaiveDate::from_ymd(2018, 3, 29),
            "creation_date should be 29/03/2018"
        )
    }

    #[test]
    fn parse_str_valid() {
        let actual = parse_str("05505");

        assert_eq!(actual.is_ok(), true, "String should be ok");
        assert_eq!(
            actual.unwrap(),
            String::from("05505"),
            "String should be 05505"
        );
    }

    #[test]
    fn parse_duplicate_valid_true() {
        let actual = parse_duplicate("D");

        assert_eq!(actual.is_ok(), true, "Duplicate 'D' should be ok");
        assert_eq!(actual.unwrap(), true, "Duplicate 'D' should be true");
    }

    #[test]
    fn parse_duplicate_valid_false() {
        let actual = parse_duplicate(" ");

        assert_eq!(actual.is_ok(), true, "Duplicate ' ' should be ok");
        assert_eq!(actual.unwrap(), false, "Duplicate ' ' should be false");
    }

    #[test]
    fn parse_duplicate_invalid() {
        let actual = parse_duplicate("B");

        assert_eq!(actual.is_ok(), false, "Duplicate 'B' should not be ok");
    }

    #[test]
    fn parse_u8_valid() {
        let actual = parse_u8("2");

        assert_eq!(actual.is_ok(), true, "u8 '2' should be ok");
        assert_eq!(actual.unwrap(), 2, "u8 '2' should be 2");
    }

    #[test]
    fn parse_u8_invalid() {
        let actual = parse_u8("200000");

        assert_eq!(actual.is_ok(), false, "u8 '200000' should not be ok");
    }

    #[test]
    fn parse_u64_valid() {
        let actual = parse_u64("20000");

        assert_eq!(actual.is_ok(), true, "u64 '20000' should be ok");
        assert_eq!(actual.unwrap(), 20000, "u64 '20000' should be 20000");
    }

    #[test]
    fn parse_u64_invalid() {
        let actual = parse_u8("200000200000200000200000200000200000");

        assert_eq!(
            actual.is_ok(),
            false,
            "u8 '200000200000200000200000200000200000' should not be ok"
        );
    }

    #[test]
    fn parse_field_valid() {
        let line_header = "0000029031872505        00099449  Testgebruiker21           KREDBEBB   00630366277 00000                                       2";
        // let range: Range<usize> = 5..11;
        let actual = parse_field(line_header, 5..11, parse_date);
        assert_eq!(actual.is_ok(), true, "Date should be ok");
        assert_eq!(
            actual.unwrap(),
            NaiveDate::from_ymd(2018, 3, 29),
            "creation_date should be 29/03/2018"
        )
    }
}