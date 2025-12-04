mod day1;
mod day2;
mod day3;
mod day4;
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
        _ => println!("Invalid day"),
    }
}
