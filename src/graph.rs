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
    pub fn draw_svg(&self) -> CodepenJson {
        // Load templates from the "templates" folder using a glob pattern
        // let mut context = Context::new();
        // //hardset the padding around the graph
        // let padding = 50;
        //
        // //ensure the viewbox is as per input
        // let width = width - padding * 2;
        // let height = height - padding * 2;
        //
        // context.insert("name", &self.name);
        // context.insert("width", &width);
        // context.insert("height", &height);
        // context.insert("padding", &padding);
        //
        // let str = r#"
        // <?xml version="1.0" standalone="no"?> \
        // <svg \
        //   width="100%" \
        //   height="100%" \
        //   viewBox="0 0 {{height + padding * 2}} {{width + padding * 2}}" \
        //   preserveAspectRatio="xMidYMid meet" \
        //   xmlns="http://www.w3.org/2000/svg" \
        //   > \
        //   <text \
        //     x="{{width/2 + padding}}" \
        //     y="{{padding / 2}}" \
        //     font-family="-apple-system, system-ui, BlinkMacSystemFont, Roboto" \
        //     dominant-baseline="middle" \
        //     text-anchor="middle" \
        //     font-size="18" \
        //     fill="red" \
        //     font-weight="700" \
        //     > \
        //     {{name}} \
        //   </text> \
        // </svg>"#;
        // let rendered = Tera::one_off(include_str!("templates/svg_template.html"), &context, true).expect("Could not draw graph");
        //let rendered = tera.render("svg_template.html", &context).unwrap();
        let json = CodepenJson {
            title: "New Pen".to_string(),
            html: "<div>Hello, World!</div>".to_string(),
        };
        println!("{}", "iummm hello???");
        return json;
    }
}
