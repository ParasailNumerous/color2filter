mod css_filter;

use clap::Parser;

#[derive(Parser)]
struct Args {
    color: String
}

fn main() {
    let args = Args::parse();
    
    match csscolorparser::parse(&args.color) {
        Ok(color) => {
            // r, g, b are between 0-1, multiply by 256
            let (r, g, b) = (
                (color.r * 256f32) as u8,
                (color.g * 256f32) as u8,
                (color.b * 256f32) as u8
            );
            let filter = css_filter::css_filter(r, g, b);
            println!("{filter}");
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(-1)
        }
    }
}
