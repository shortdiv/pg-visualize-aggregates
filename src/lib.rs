mod graph;
mod models;
mod seed;

use pgrx::prelude::*;

use anyhow::Result;
use reqwest::{blocking::Client, RequestBuilder};
use serde_json;
use std::collections::HashMap;

use crate::graph::Graph;

::pgrx::pg_module_magic!();

const SVG: &str = include_str!("templates/svg_template.html");

#[pg_extern]
fn hello_visualizing_aggregates() -> &'static str {
    "Hello, visualizing_aggregates"
}

#[pg_extern]
fn draw_graph() -> Result<String> {
    let graph = Graph::new("Example".to_string(), 40, "#8ff0a4".into());
    let svg = graph.draw_svg(SVG);
    let mut form = HashMap::new();
    form.insert("title", svg.title);
    form.insert("html", svg.html);

    let dat1 = serde_json::to_string(&form).unwrap();

    pgrx::info!("{}", dat1);

    // let dat: String = format!(r#"{{"title": {}, "html": {}}}"#, svg_title, svg_html);
    let form_data = format!("data={}", dat1);
    // pgrx::info!("{}", svg);

    let client = Client::new();

    let res = client
        .post("https://codepen.io/pen/define")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()?;

    // pgrx::info!("{:?}", res.body());

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

    // #[pg_test]
    // // fn test_insert() {
    // //     // tbh not sure how to run this in the extension
    // //     assert!(crate::insert_climb_data().contains("INSERT"));
    // //     assert!(crate::insert_climb_data().contains("VALUES"));
    // // }
    // #[pg_test]
    // fn test_iterator() {
    //     assert_eq!(1, crate::to_day("1 Jul 2003 10:52:37 +0200"))
    // }
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
