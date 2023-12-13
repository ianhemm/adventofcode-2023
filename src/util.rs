use std::{fs::File, io::Read};

pub fn aoc_getdata(url: &str) -> std::io::Result<String> {
    let mut out: String = String::new();
    let _ = File::open(format!("data/{}", url))?.read_to_string(&mut out);
    Ok(out)
}
