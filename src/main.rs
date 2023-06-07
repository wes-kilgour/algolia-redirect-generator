extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Rule {
    enabled: bool,
    conditions: Vec<Condition>,
    consequence: Consequence,
    object_id: Uuid,
}

#[derive(Debug, Serialize)]
struct Condition {
    anchoring: String,
    pattern: String,
    alternatives: bool,
}

#[derive(Debug, Serialize)]
struct Consequence {
    params: Params,
    filter_promotes: bool,
}

#[derive(Debug, Serialize)]
struct Params {
    rendering_content: RenderingContent,
}

#[derive(Debug, Serialize)]
struct RenderingContent {
    redirect: Redirect,
}

#[derive(Debug, Serialize)]
struct Redirect {
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("Please enter the path to the csv file: ");
    io::stdout().flush().unwrap();
    let mut path_str = String::new();
    io::stdin().read_line(&mut path_str).expect("Failed to read line");

    let path = Path::new(path_str.trim());
    let file = File::open(&path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut rules = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let search_term = record.get(0).unwrap().to_string();
        let redirect_url = record.get(1).unwrap().to_string();

        let rule = Rule {
            enabled: true,
            conditions: vec![
                Condition {
                    anchoring: "is".to_string(),
                    pattern: search_term,
                    alternatives: true,
                }
            ],
            consequence: Consequence {
                params: Params {
                    rendering_content: RenderingContent {
                        redirect: Redirect { url: redirect_url },
                    }
                },
                filter_promotes: true,
            },
            object_id: Uuid::new_v4(),
        };

        rules.push(rule);
    }

    let output_path = Path::new("rules.json");
    let file = File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &rules)?;

    println!("File created: {}", output_path.display());

    Ok(())
}

