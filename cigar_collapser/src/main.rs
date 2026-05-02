
use std::env;
use std::process::ExitCode;
use std::io::{self, BufRead};

use cigar_collapser::collapse_cigar;


enum Mode {
    Args,
    Stdin,
}


fn main() -> ExitCode {
    let mode = detect_mode();
    let exit_code = match mode {
        Mode::Args => {
            run_args_mode()
        }
        Mode::Stdin => {
            run_stdin_mode()
        }
    };

    exit_code
}

fn detect_mode() -> Mode {
    match env::args().nth(1) {
        None => {
            return Mode::Stdin;
        }
        _ => {
            return Mode::Args;
        }
    }
}


fn run_args_mode() -> ExitCode {
    let mut work_args = env::args(); 
    work_args.next(); // pass slement 0

    for arg in work_args {
        match collapse_cigar(&arg) {
            Ok(ok_str) => {
                println!("{}", ok_str);
            },
            Err(err_str) => {
                eprintln!("{}", err_str);
                return ExitCode::FAILURE;
            },
        }
    }

    ExitCode::SUCCESS
}

fn run_stdin_mode() -> ExitCode  {
    let reader = io::stdin().lock();

    for line_result in reader.lines() {
        if let Err(error) = line_result {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
        let line = line_result.unwrap();

        if line.is_empty() {
            continue;
        }

        match collapse_cigar(&line) {
            Ok(ok_str) => {
                println!("{}", ok_str);
            },
            Err(err_str) => {
                eprintln!("{}", err_str);
                return ExitCode::FAILURE;
            },
        }
    }

    ExitCode::SUCCESS
}
