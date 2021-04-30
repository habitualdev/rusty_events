extern crate win_event_log;


use win_event_log::prelude::*;
use regex;
use std::{
    fs::File,
    io::{self,prelude::*, BufReader},
    path::Path,
    env,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn load_config(config_lines: vec) {
    let lines = lines_from_file("/env.conf");
    for line in lines {
        let mut string_delim = line.split(":").collect();
        println!(string_delim)
    }

}



fn send_http(msg: &str) {
    let string = msg;
    let client = reqwest::Client::new();
    let config = std::fs::read_to_string("env.conf");
    }



fn main() {
    let conditions = vec![
        Condition::filter(EventFilter::level(1, Comparison::Equal)),
        Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
    ];
    let query = QueryList::new()
        .with_query(
            Query::new()
                .item(
                    QueryItem::selector("Application".to_owned())
                        .system_conditions(Condition::or(conditions))
                        .build(),
                )
                .query(),
        )
        .build();

    match WinEvents::get(query) {
        Ok(events) => {
            if let Some(event) = events.into_iter().next() {
                println!("{}", event);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
