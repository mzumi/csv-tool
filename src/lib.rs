extern crate csv;
extern crate pbr;
extern crate regex;
extern crate prettytable;

use regex::*;
use std::fs;

use pbr::{ProgressBar, Units};
use prettytable::Table;

#[derive(Debug)]
pub struct CSVTools {}

impl CSVTools {
    pub fn convert(from: String, to: String, source: String, output: String) {
        let converter = Convertor::new(from.to_owned(),
                                       to.to_owned(),
                                       source.to_owned(),
                                       output.to_owned());
        converter.run();
    }

    pub fn view(source: String) {
        let viewer = Viewer::new(source.to_owned());
        viewer.run();
    }
}

#[derive(Debug)]
struct Convertor {
    regex: Result<Regex, Error>,
    target: String,
    source: String,
    output: String,
}

impl Convertor {
    pub fn new(from: String, to: String, source: String, output: String) -> Self {
        Convertor {
            regex: Regex::new(from.as_str()),
            target: to,
            source: source,
            output: output,
        }
    }

    fn run(&self) {
        if let Ok(ref re) = self.regex {
            let mut writer = csv::Writer::from_file(&self.output).unwrap();

            let mut reader = csv::Reader::from_file(&self.source).unwrap();

            let n_bytes = fs::metadata(&self.source).unwrap().len() as usize;
            let mut pb = ProgressBar::new(n_bytes as u64);
            pb.set_units(Units::Bytes);

            let mut copied_size: usize = 0;
            for h in reader.headers() {
                copied_size += h.iter().fold(0, |sum, s| sum + s.capacity());
                let _ = writer.write(h.into_iter());
                pb.set(copied_size as u64);
            }

            for row in reader.records().map(|r| r.unwrap()) {
                copied_size += row.iter().fold(0, |sum, s| sum + s.capacity());

                let after = row.iter()
                    .map(|r| re.replace_all(r.as_str(), self.target.as_str()));
                let _ = writer.write(after.into_iter());
                pb.set(copied_size as u64);
            }
            pb.finish();
        }
    }
}

#[derive(Debug)]
struct Viewer {
    source: String,
}

impl Viewer {
    pub fn new(source: String) -> Self {
        Viewer { source: source }
    }
    pub fn run(&self) {
        let table = Table::from_csv_file(&self.source).unwrap();
        table.printstd();
    }
}
