// Rust library, inspired by Golang's Colly, for web scraping.
use fantoccini::*;
use thirtyfour::prelude::*;

pub struct Scraper {
  pub client: thirtyfour::WebDriver,
  pub queue: Queue,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
