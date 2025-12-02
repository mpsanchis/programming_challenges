use std::env;

#[derive(Debug)]
pub enum Part {
    One,
    Two,
}

#[derive(Debug)]
pub struct Args {
    pub verbosity: bool,
    pub day: u8,
    pub part: Part,
}

pub fn parse_args() -> Args {
    let raw_args: Vec<String> = env::args().collect();

    let mut args = Args {
        verbosity: false,
        day: 1,
        part: Part::One,
    };

    for arg in raw_args.iter().skip(1) {
        let arg_str = arg.as_str();

        if arg_str == "--verbose" || arg_str == "-v" {
            args.verbosity = true;
        }
        if arg_str.starts_with("-d") {
            let day: u8 = arg_str[2..].parse().expect("could not parse day");
            args.day = day;
        }
        if arg_str.starts_with("-p") {
            let part: u8 = arg_str[2..].parse().expect("could not parse part");
            args.part = match part {
                1 => Part::One,
                2 => Part::Two,
                _ => panic!("Wrong part when parsing args (not 1 or 2)"),
            };
        }
    }
    args
}