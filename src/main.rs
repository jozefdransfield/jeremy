mod frames;
mod image_functions;
mod tests;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::frames::{all_frames, frame};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Frame {
        input: PathBuf,

        output: Option<PathBuf>,

        #[arg(short, long)]
        variant: Option<String>,
    },
    Info {},
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Frame {
            input,
            output,
            variant,
        }) => {
            frame(input.clone(), variant.clone(), output.clone().unwrap_or(PathBuf::from("output.png")));
        }
        Some(Commands::Info {}) => {
            all_frames().iter().for_each(|frame| println!("{} - {} - ({}x{})", frame.device, frame.variant, frame.dimensions.0, frame.dimensions.0));
        }
        None => {}
    }

    // frame --variant = "Black Titanium" --output target/iPhone 16 Pro Max - Portrait.png
}
