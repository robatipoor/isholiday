use colored::*;
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Day {
    events: Vec<String>,
    isHoliday: bool,
    date: String,
    gdate: String,
}

const URL: &'static str = "https://pholiday.herokuapp.com/today";

fn main() {
    let mut resp: Response = Client::new()
        .get(URL)
        .send()
        .expect("send requset GET faild !");
    if resp.status().is_success() {
        let d: Day = resp.json().expect("deserlialize to Day struct faild !");
        if d.isHoliday {
            println!("{} {}", d.date, "Today is a holiday ;) ".green())
        } else {
            println!("{} {}", d.date, "Today is not closed :| ".red())
        }
    }
}
