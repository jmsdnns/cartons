use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

fn make_dates() {
    let date = NaiveDate::from_ymd_opt(2024, 4, 20).unwrap();
    println!("Date: {}", date);

    let time = NaiveTime::from_hms_opt(11, 22, 33).unwrap();
    println!("Time: {}", time);

    let datetime = NaiveDateTime::new(date, time);
    println!("DT: {}", datetime);

    let utc_datetime = Utc::now();
    println!("UTC DT: {}", utc_datetime);

    let seconds = Utc::now().timestamp();
    println!("UTC ms: {}", seconds);
}

fn parse_dates() {
    let date = NaiveDate::parse_from_str("2024-09-15", "%Y-%m-%d").unwrap();
    println!("Date: {}", date);

    let formatted_d = date.format("%A, %B %e, %Y");
    println!("Formatted D: {}", formatted_d);

    let date_8601 = "2024-09-15T12:34:56Z";
    let parsed_8601 = DateTime::parse_from_rfc3339(date_8601).unwrap();
    println!("From 8601: {}", parsed_8601);

    let formatted_dt = parsed_8601.format("%Y-%m-%dT%H:%M:%S%z");
    println!("Formatted DT: {}", formatted_dt);

    let seconds = Utc::now().timestamp();
    let parsed_seconds = DateTime::from_timestamp(seconds, 0);
    println!("Parsed Unix: {:?}", parsed_seconds);
}

fn date_math() {
    let d1 = NaiveDate::from_ymd_opt(2024, 4, 20).unwrap();
    let d2 = NaiveDate::from_ymd_opt(2024, 5, 4).unwrap();

    let d3 = d1 + Duration::days(7);
    println!("D1 + a week: {}", d3);

    if d1 < d2 {
        println!("{} is before {}", d1, d2);
    }

    let duration = d2 - d1;
    println!(
        "Time between {} and {}: {} days",
        d1,
        d2,
        duration.num_days()
    );
}

fn main() {
    make_dates();
    parse_dates();
    date_math();
}
