mod logger;
mod arg_parser;

pub use logger::{init_logger, logger};
pub use arg_parser::{parse_args, Part};