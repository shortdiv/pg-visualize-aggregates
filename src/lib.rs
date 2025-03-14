mod graph;
mod models;
mod seed;

use pgrx::prelude::*;

use anyhow::Result;
use reqwest::blocking::Client;
use serde_json;
use std::collections::HashMap;

use crate::graph::Graph;

::pgrx::pg_module_magic!();

const SVG: &str = include_str!("templates/bar_graph_template.html");

#[pg_extern]
fn hello_visualizing_aggregates() -> &'static str {
    "Hello, visualizing_aggregates"
}

pub fn fetch_data(_dataset: String, batch_size: i32) -> Result<pgrx::Json> {
    pgrx::info!("logging here");
    let query = format!("SELECT * FROM climbs LIMIT {}", batch_size);
    pgrx::info!("{}", query);
    Ok(Spi::connect(|client| {
        let query = r#"SELECT * FROM climbs LIMIT $1"#;
        pgrx::info!("{}", query);
        client
            .select(query, Some(1), &[batch_size.into()])?
            .get_one::<pgrx::Json>()
        // let mut cursor = client.open_cursor(&query, &[]);
        // cursor.fetch(1)?.get_one::<pgrx::Json>()
    })
    .unwrap()
    .unwrap())
}

#[pg_extern]
fn draw_graph(dataset: String) -> Result<String> {
    pgrx::info!("heloo????");
    let _data = fetch_data(dataset, 1).unwrap();
    // println!("{:?}", data);
    let graph = Graph::new("Example".to_string(), 420, "#8ff0a4".into());
    let svg = graph.draw_svg(SVG, 150, 10);
    let mut form = HashMap::new();
    form.insert("title", svg.title);
    form.insert("html", svg.html);

    let dat1 = serde_json::to_string(&form).unwrap();

    let form_data = format!("data={}", dat1);

    let client = Client::new();

    let res = client
        .post("https://codepen.io/pen/define")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()?;

    Ok(res.url().to_string())
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
