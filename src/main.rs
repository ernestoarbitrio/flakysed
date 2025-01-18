use clap::Parser;
use flakysed::ProcessFile;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, help = "Path to the CircleCI log file to process")]
    input_file: String,

    #[arg(short, long, help = "Path to save the cleaned and processed output")]
    output_file: String,

    #[arg(
        short,
        long,
        help = "Worker name to filter logs for processing e.g. gw7"
    )]
    worker: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = ProcessFile::run(&args.input_file, &args.output_file, &args.worker) {
        eprintln!("Error: Failed to process file - {e}");
        std::process::exit(1);
    }

    println!(
        "File processed successfully. Output saved to '{}'.",
        args.output_file
    );
}
