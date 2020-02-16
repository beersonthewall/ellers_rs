mod maze;
use std::env;

fn print_help() {
    println!("Eller's maze generation algorithm implementation.");
    println!("");
    println!("USAGE:");
    println!("      gen [width] [iterations]");
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-help") {
        print_help();
    }
    if args.len() != 3 {
        println!("Error");
    }
    let width = usize::from_str_radix(&args[1], 10).unwrap();
    let iterations = usize::from_str_radix(&args[2], 10).unwrap();

    let mut builder = maze::MazeBuilder::new(width);
    builder.print_row();

    for _ in 0..iterations - 2 {
        builder.ellers();
        builder.print_row();
    }

    builder.end();
    builder.print_row();
}
