use clap::Parser;

#[derive(Parser)]
#[command(name = "echor")]
#[command(author = "zuya@microsoft.com")]
#[command(version)]
#[command(about = "Rust echo", long_about = None)]
struct Cli {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    print!(
        "{}{}",
        cli.text.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    );
}
