use clap::Parser;

#[derive(Parser)]
#[command(version, about ="CLI File Organizer in Rust")]
struct Cli {
    /// The path to the directory to organize
    #[arg(short, long)]
    path: String,

    /// Organizing mode: [by_extension, by_date]
    #[arg(short, long)]
    mode: String,
}

fn main() {
    let args = Cli::parse();
    println!("Organizing {} using mode: {}", args.path, args.mode);

}
