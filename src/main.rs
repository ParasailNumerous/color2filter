use clap::Parser;

use color2filter::color_to_filter;

#[derive(Parser)]
struct Args {
    color: String
}

fn main() {
    let args = Args::parse();
    
    match color_to_filter(&args.color) {
        Ok(filter) => {
            println!("{filter}");
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
}
