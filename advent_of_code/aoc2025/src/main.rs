mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod util;

fn main() {
    let args = util::parse_args();
    util::init_logger(args.verbosity);

    if args.verbosity {
        util::logger().logn(&format!("Args: {:?}", args));
    }

    match args.day {
        1 => day1::main(&args.part),
        2 => day2::main(&args.part),
        3 => day3::main(&args.part),
        4 => day4::main(&args.part),
        5 => day5::main(&args.part),
        6 => day6::main(&args.part),
        7 => day7::main(&args.part),
        8 => day8::main(&args.part),
        9 => day9::main(&args.part),
        10 => day10::main(&args.part),
        _ => println!("Invalid day"),
    }
}
