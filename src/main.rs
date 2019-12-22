#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate regex;

use prettytable::Table;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

use regex::Regex;

fn main() {
    let config_file = dirs::home_dir()
        .unwrap()
        .join(Path::new(".config/mimeapps.list"));

    let mut configuration = String::from("[Default Applications]\n");

    let mime_type_app_map = collect_mime_types("/usr/share/applications");

    println!("Set default application for mime types with no alternatives.");

    let mut table = Table::new();
    table.add_row(row!["Mime type", "Application"]);
    for (mime_type, apps) in mime_type_app_map.iter().filter(|(_, x)| x.len() == 1) {
        table.add_row(row![mime_type, apps[0].trim_end_matches(".desktop")]);
        configuration += &format!("{}={}\n", mime_type, apps[0]);
    }
    table.printstd();

    let multi_app: HashMap<_, _> = mime_type_app_map
        .iter()
        .filter(|(_, x)| x.len() > 1)
        .map(|(x, y)| (x.to_owned(), y.to_owned()))
        .collect();
    let app_groups = pivot(multi_app);
    let len = app_groups.len();
    let mut i = 0;

    for (apps, mime_types) in app_groups {
        i += 1;
        println!("Group {} of {}", i, len);
        println!("Mime types: {}", mime_types.join(", "));
        println!("Competitors:");
        let mut j = 0;
        for app in &apps {
            j += 1;
            println!("\t{}. {}", j, app);
        }
        if let Some(choice) = get_input(apps.len()) {
            for mime_type in mime_types {
                configuration += &format!("{}={}\n", mime_type, apps[choice - 1])
            }
        }
    }

    fs::write(&config_file, configuration).unwrap();
    println!(
        "Configuration is saved to {}",
        config_file.to_str().unwrap()
    );
}

fn pivot(mime_type_app_map: HashMap<String, Vec<String>>) -> HashMap<Vec<String>, Vec<String>> {
    let mut pivoted = HashMap::new();
    for (mime_type, apps) in mime_type_app_map {
        pivoted.entry(apps).or_insert_with(Vec::new).push(mime_type);
    }
    pivoted
}

fn get_input(count: usize) -> Option<usize> {
    loop {
        print!("Choose app [1-{}, 0 to skip]: ", count);
        let _ = io::stdout().flush();
        let mut answer = String::new();
        let _ = io::stdin().read_line(&mut answer);
        match answer.trim_end_matches('\n').parse::<usize>() {
            Ok(n) => {
                if n >= 1 && n <= count {
                    return Some(n);
                } else if n == 0 {
                    return None;
                } else {
                    println!("What? Select a number from 0 to {}.", count);
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}

fn collect_mime_types(dir: &str) -> HashMap<String, Vec<String>> {
    let paths = fs::read_dir(dir).unwrap();
    let mut mime_types: HashMap<String, Vec<String>> = HashMap::new();
    for path in paths {
        let path = path.unwrap().path();
        for mime in get_mime_types_from_file(path.to_str().unwrap()) {
            mime_types
                .entry(mime)
                .or_insert_with(Vec::new)
                .push(String::from(path.file_name().unwrap().to_str().unwrap()));
        }
    }
    mime_types
}

fn get_mime_types_from_file(app: &str) -> Vec<String> {
    let f = File::open(app).expect("Unable to open file");
    let f = BufReader::new(f);

    let re = Regex::new(r"^MimeType=(.*)$").unwrap();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if let Some(mime_types) = re.captures(&line) {
            return mime_types
                .get(1)
                .unwrap()
                .as_str()
                .split(';')
                .filter(|x| !x.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
        }
    }
    Vec::new()
}
