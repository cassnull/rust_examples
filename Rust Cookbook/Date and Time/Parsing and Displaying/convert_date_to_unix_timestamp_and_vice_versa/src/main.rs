use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd_opt(2017, 11, 12)
        .unwrap()
        .and_hms_opt(17, 33, 44)
        .unwrap();
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time,
        date_time.timestamp()
    );

    let date_time_after_a_billion_seconds =
        NaiveDateTime::from_timestamp_opt(1_000_000_000, 0).unwrap();
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds
    );
}
