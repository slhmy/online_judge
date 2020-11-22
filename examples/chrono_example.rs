use chrono::NaiveDate;

fn main() {
    let dt = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
    println!("{}", dt.timestamp());

    let dt = NaiveDate::from_ymd(2020, 11, 15).and_hms(20, 0, 0);
    println!("{}", dt.timestamp());

    let dt = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    assert_eq!(dt, NaiveDate::from_ymd(2001, 9, 9).and_hms(1, 46, 40));
}