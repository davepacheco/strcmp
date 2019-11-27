/*
 * strcmp: command-line tool to compare two strings given as command-line arguments.  The canonical
 * use-case is passing to checksum strings to determine if they match.
 */
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("strcmp: usage: strcmp STR1 STR2");
        process::exit(2);
    }

    if args[1] == args[2] {
        println!("strcmp: arguments match (\"{}\")", args[1]);
        process::exit(0);
    }

    println!("strcmp: argument mismatch");
    println!("arg1: {}", args[1]);
    println!("arg2: {}", args[2]);
    process::exit(1);
}
