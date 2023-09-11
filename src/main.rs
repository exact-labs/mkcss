mod css;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use colored::Colorize;
use inquire::Text;

fn main() {
    #[derive(Parser)]
    #[command(version)]
    struct Cli {
        /// File path
        path: Option<String>,
        /// Add reset.css to file
        #[arg(short, long)]
        reset: bool,
        #[clap(flatten)]
        verbose: Verbosity,
    }

    let cli = Cli::parse();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();

    if let Some(path) = cli.path.as_deref() {
        css::write(path, cli.reset).unwrap();
    } else {
        match Text::new("path").with_default("index.html").with_help_message("The path of your HTML file.").prompt() {
            Ok(path) => css::write(&path, cli.reset).unwrap(),
            Err(_) => println!("{}", "Aborting...".white()),
        }
    }
}
