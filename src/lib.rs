mod graph;
mod models;
mod seed;

use pgrx::prelude::*;

use anyhow::Result;
use reqwest::blocking::Client;
use serde_json;

use crate::graph::Graph;
use crate::seed::load_climb_data;

::pgrx::pg_module_magic!();

#[pg_extern]
fn hello_visualizing_aggregates() -> &'static str {
    "Hello, visualizing_aggregates"
}

#[pg_extern]
fn draw_graph() -> Result<String> {
    let graph = Graph::new("Example".to_string(), 40, "#8ff0a4".into());
    let svg = graph.draw_svg();
    let svg_title = serde_json::to_string(&svg.title).unwrap();
    let svg_html = serde_json::to_string(&svg.html).unwrap();

    let dat: String = format!(r#"{{"title": {}, "html": {}}}"#, svg_title, svg_html);
    let form_data = format!("data={}", dat);

    let client = Client::new();
    let res = client
        .post("https://codepen.io/pen/define")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data.clone())
        .send()?;

    Ok(res.url().to_string())
}

#[pg_extern]
fn insert_climb_data() -> String {
    let climbs = load_climb_data();
    let keys = [
        "id",
        "route_name",
        "parent_sector",
        "route_id",
        "sector_id",
        "type_string",
        "fa",
        "yds",
        "vermin",
        "nopm_yds",
        "nopm_vermin",
        "yds_rank",
        "vermin_rank",
        "safety",
        "parent_loc",
        "description",
        "location",
        "protection",
        "corrected_users_ratings",
    ];
    let mut climb_values = vec![];

    for climb in &climbs {
        let mut vals: Vec<String> = vec![];
        for k in &keys {
            let serialized = serde_json::to_value(&climb).unwrap();
            vals.push(serialized[k].to_string());
        }
        climb_values.push(format!("({})", vals.join(",")));
    }
    return format!(
        "INSERT INTO climbs({}) VALUES {}",
        keys.join(","),
        climb_values.join(",")
    );
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
    fn test_insert() {
        // tbh not sure how to run this in the extension
        assert!(crate::insert_climb_data().contains("INSERT"));
        assert!(crate::insert_climb_data().contains("VALUES"));
    }

    #[pg_test]
    fn test_iterator() {
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
