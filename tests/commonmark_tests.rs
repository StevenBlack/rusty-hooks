#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
struct CommonmarkTestCase {
  markdown: String,
  html: String,
  example: usize,
  start_line: usize,
  end_line: usize,
  section: String
}

impl fmt::Display for CommonmarkTestCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "example: {} ({})", self.example, self.section)
    }
}

#[datatest::data("tests/commonmark_tests.json")]
fn sample_test(case: CommonmarkTestCase) {
  assert_eq!(case.html, case.markdown);
}