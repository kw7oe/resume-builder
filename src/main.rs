use handlebars::Handlebars;
use std::fs::File;

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("template", include_str!("resume.hbs"))
        .unwrap();
    let data: serde_json::Value = serde_yaml::from_str(include_str!("data.yml")).unwrap();

    let mut output_file = File::create("target/resume.html").unwrap();
    handlebars
        .render_to_write("template", &data, &mut output_file)
        .unwrap();
}
