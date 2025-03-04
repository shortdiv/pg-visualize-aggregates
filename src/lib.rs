use chrono::{DateTime, Datelike, Weekday};
use pgrx::prelude::*;

::pgrx::pg_module_magic!();

#[pg_extern]
fn hello_visualizing_aggregates() -> &'static str {
    "Hello, visualizing_aggregates"
}

#[pg_extern]
fn to_day(date: &str) -> i32 {
    match DateTime::parse_from_str(date, "%d %b %Y %H:%M:%S %z") {
        Ok(parsed_date) => parsed_date.weekday().num_days_from_monday() as i32,
        Err(_) => -1,
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_visualizing_aggregates() {
        assert_eq!(
            "Hello, visualizing_aggregates",
            crate::hello_visualizing_aggregates()
        );
    }

    #[pg_test]
    fn test_date_to_day() {
        assert_eq!(1, crate::to_day("1 Jul 2003 10:52:37 +0200"))
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
