use clap::Parser;

#[derive(Parser)]
struct Compiler {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Compiler::parse();

    if args.verbose {
        println!("Verbose mode enabled");
    } else {
        println!("Verbose mode disabled");
    }
}
