mod frames;
mod image_functions;
mod tests;
mod templates;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::frames::{all_frames, frame};
use crate::templates::render;

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
    Render {
        template: PathBuf,
        screenshot: PathBuf,
        output: PathBuf,
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
            frame(
                input.clone(),
                variant.clone(),
                output.clone().unwrap_or(PathBuf::from("output.png")),
            );
        }
        Some(Commands::Render {
            template,
            screenshot,
            output,
        }) => {
            render(template.clone(), screenshot.clone(), output.clone())
        }
        Some(Commands::Info {}) => {
            all_frames().iter().for_each(|frame| {
                println!(
                    "{} - {} - ({}x{})",
                    frame.device, frame.variant, frame.dimensions.0, frame.dimensions.0
                )
            });
        }
        None => {}
    }
}



