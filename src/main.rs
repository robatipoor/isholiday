use colored::*;
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Day {
    events: Vec<String>,
    isHoliday: bool,
    date: String,
    gdate: String,
}

fn main() {
    let url = "https://pholiday.herokuapp.com/today";
    let mut resp: Response = Client::new().get(url).send().unwrap();
    if resp.status().is_success() {
        let d: Day = resp.json().unwrap();
        if d.isHoliday {
            println!("{} {}", d.date,"Today is a holiday".green())
        } else {
            println!("{} {}", d.date,"Today is not closed".red())
        }
    }
}

