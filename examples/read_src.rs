use std::fs::File;
use std::io::prelude::*;
use serde_json::json;

fn main() -> std::io::Result<()> {
    let mut file = File::open("c_src.c")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let info = json!({
        "src": contents
    });
    println!("{}", info.to_string());
    Ok(())
}