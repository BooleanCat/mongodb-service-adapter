use std::{env, process};

const ERROR_EXIT_CODE: i32 = 1;
const NOT_IMPLEMENTED_EXIT_CODE: i32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        process::exit(ERROR_EXIT_CODE);
    }

    let command: &str = &args[1][..];

    match command {
        "generate-manifest" => process::exit(NOT_IMPLEMENTED_EXIT_CODE),
        "create-binding"    => process::exit(NOT_IMPLEMENTED_EXIT_CODE),
        "delete-binding"    => process::exit(NOT_IMPLEMENTED_EXIT_CODE),
        "dashboard-url"     => process::exit(NOT_IMPLEMENTED_EXIT_CODE),
        _                   => process::exit(ERROR_EXIT_CODE),
    }
}
