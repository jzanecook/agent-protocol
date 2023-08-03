use clap::{Parser, Subcommand};
use colored::Colorize;

/// Agent Protocol (built by e2b)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help = "Try agent-protocol help <command> for more information")]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Test compliance of an agent
    Test {
        /// The URL of the agent
        #[arg(short, long)]
        url: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        true => println!("Debugging Enabled"),
        _ => (),
    }

    match &cli.command {
        Some(Commands::Test { url }) => {
            if url.is_some() {
                println!("Testing agent at {}", url.as_ref().unwrap())
            } else {
                println!(
                    "{error_msg}\n\n{usage} {agent_arg} test {url_arg}",
                    error_msg = "Error: No URL specified".bold().red(),
                    usage = "Usage:".bold().underline(),
                    agent_arg = "agent-protocol".bold(),
                    url_arg = "--url <url>".underline().italic()
                );
                std::process::exit(1)
            }
        }
        None => println!("No command specified"),
    }
}
