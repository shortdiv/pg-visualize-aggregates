use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub width: i32,
    pub colour: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodepenJson {
    pub title: String,
    pub html: String,
}

impl Graph {
    pub fn new(name: String, width: i32, colour: String) -> Self {
        Graph {
            name,
            width,
            colour,
        }
    }
    pub fn draw_svg(&self, template: &str) -> CodepenJson {
        let mut env = Environment::new();
        env.add_template("svg", template).unwrap();
        let tmpl = env.get_template("svg").unwrap();
        let rendered = tmpl.render(context!(width => 40, height => 40)).unwrap();

        let json = CodepenJson {
            title: "This is a Pen".to_string(),
            html: rendered,
        };
        return json;
    }
}
