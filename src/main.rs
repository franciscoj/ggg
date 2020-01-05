mod commands;
mod github;

use clap::{App, SubCommand};

#[tokio::main]
async fn main() {
    env_logger::init();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

    let matches = App::new("ggg: A Github CLI")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .subcommand(
            SubCommand::with_name("notifications").about("Shows unread notifications count"),
        )
        .subcommand(
            SubCommand::with_name("list_notifications")
                .about("Shows unread notifications list per repo"),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("notifications") {
        commands::notifications::cmd();
    }

    if let Some(_matches) = matches.subcommand_matches("list_notifications") {
        commands::list_notifications::cmd();
    }
}
