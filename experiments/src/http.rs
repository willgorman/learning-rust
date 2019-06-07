extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::panic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
  slideshow: Slideshow,
}

#[derive(Debug, Serialize, Deserialize)]
struct Slideshow {
  author: String,
  title: String,
  date: String,
  slides: Vec<Slide>
}

#[derive(Debug, Serialize, Deserialize)]
struct Slide {
  title: String,
  #[serde(rename = "type")]
  stype: String,
  items: Vec<String>
}

fn get() -> Result<(), reqwest::Error> {
  let rsp = reqwest::Client::new()
  .get("https://httpbin.org/get").send();
  // let rslt = panic::catch_unwind(|| {
  //   let _ = rsp.unwrap().status();
  // });
  println!("GET status {:?}", rsp?.status());
  Ok(())
}

fn get_json() -> Result<(), reqwest::Error> {
  let slideshow = reqwest::Client::new()
  .get("https://httpbin.org/json").send();

  Ok(())
}

#[cfg(test)]
mod tests {
  #[test]
  fn name() {
      let _ = super::get();
  }

  #[test]
  fn get_json() {
      unimplemented!();
  }
}
