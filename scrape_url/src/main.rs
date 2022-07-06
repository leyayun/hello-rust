
use std::fs;

fn scrape_url(url: &str, output: &str){
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}

fn pi() -> f64 {
  3.1415926
}

fn not_pi() {
  3.1415926;
}

fn main() {
  let is_pi = pi();
  let is_unit1 = not_pi();
  let is_unit2 = {
    pi();
  };
  
  scrape_url("https://www.rust-lang.org/", "rust.md");
  println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}