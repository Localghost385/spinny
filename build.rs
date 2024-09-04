use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use tera::{Context, Tera};
use toml::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let mut tera = Tera::default();
    tera.add_template_file("src/shapes/shape.tera", Some("shape.tera"))?;

    let mut file = File::open("src/shapes/shapes.toml")?;
    let mut toml_content = String::new();
    file.read_to_string(&mut toml_content)?;
    let shapes: Value = toml_content.parse::<Value>()?;

    let output_path = Path::new(&out_dir).join("shape.rs");
    let mut context = Context::new();
    context.insert("shapes", &shapes);

    let rust_code = tera.render("shape.tera", &context)?;
    fs::write(output_path, rust_code)?;

    Ok(())
}
