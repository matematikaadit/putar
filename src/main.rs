#[macro_use] extern crate tera;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;

use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use tera::{Tera, Context};
use regex::Regex;
use chrono::Utc;

lazy_static! {
    pub static ref TEMPLATES: Tera = compile_templates!("templates/**/*");
}

fn main() {
    for arg in env::args().skip(1) {
        let input = Path::new(&arg);
        let stem = input.file_stem().unwrap().to_str().unwrap();
        let output = Path::new(stem).with_extension("html");

        // preparing metadata
        let (date, channel) = parse_stem(&stem);
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

        // processing the logs
        let f = File::open(input).unwrap();
        let buf = BufReader::new(f);
        let logs: Vec<_> = buf.lines().map(|l| l.unwrap()).collect();

        // preparing context
        let mut context = Context::new();
        context.add("channel", &channel);
        context.add("date", &date);
        context.add("logs", &logs);
        context.add("now", &now);

        match TEMPLATES.render("main.html", &context) {
            Ok(s) => {
                eprintln!("WRITING {:?}", output);
                let mut file = File::create(output).unwrap();
                file.write_all(s.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                for e in e.iter().skip(1) {
                    eprintln!("REASON: {}", e);
                }
            }
        }
    }
}

fn parse_stem(stem: &str) -> (&str, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d{4}-\d{2}-\d{2})-(.+)").unwrap();
    }
    let caps = RE.captures(stem).unwrap();
    let date    = caps.get(1).unwrap().as_str();
    let channel = caps.get(2).unwrap().as_str();
    (date, channel)
}
