extern crate csv_tools;
extern crate clap;

use csv_tools::*;
use clap::*;

fn main() {
    let matches = App::new("rc")
        .version("0.1")
        .author("mzumi")
        .about("CSV Tools")
        .subcommand(SubCommand::with_name("convert")
            .about("Convert contents of CSV file")
            .version("0.1")
            .arg(Arg::with_name("from")
                .help("Sets the string to be converted")
                .required(true)
                .index(1))
            .arg(Arg::with_name("to")
                .help("Sets the converted string")
                .required(true)
                .index(2))
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(3))
            .arg(Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .required(true)
                .index(4)))
        .subcommand(SubCommand::with_name("view")
            .about("Show contents of CSV file")
            .version("0.1")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("convert") {
        let from = matches.value_of("from").unwrap();
        let to = matches.value_of("to").unwrap();
        let input = matches.value_of("INPUT").unwrap();
        let output = matches.value_of("OUTPUT").unwrap();

        CSVTools::convert(from.to_owned(),
                          to.to_owned(),
                          input.to_owned(),
                          output.to_owned());
    } else if let Some(matches) = matches.subcommand_matches("view") {
        let input = matches.value_of("INPUT").unwrap();
        CSVTools::view(input.to_owned());
    }
}
