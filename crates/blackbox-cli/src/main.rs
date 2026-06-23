use clap::Parser;

#[derive(Parser)]
#[command(name = "blackbox")]
enum Cli {
    Wizard,
    Dev,
    Serve {
        #[arg(long)]
        service: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Wizard => println!("wizard: not yet implemented"),
        Cli::Dev => println!("dev: not yet implemented"),
        Cli::Serve { service } => println!("serve {}: not yet implemented", service),
    }
}
