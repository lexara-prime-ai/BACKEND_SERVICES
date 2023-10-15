extern crate cr8s;

use clap::{Arg, Command};


fn main() {
    // Define available sub commands
    // User sub commands

    // To Do:: Implement .short & .long

    let matches = Command::new("cr8s")
        .about("Cr8s commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a user with multiple roles attached...")
                        .arg_required_else_help(true)
                        // Define arguments
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        // Define multiple arguments, args -> 1.. inclusive
                        // Specify/provide a value_delimiter a.k.a value separator
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                )
                .subcommand(
                    Command::new("list")
                        .about("List all available users")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by ID")
                        .arg(Arg::new("id").required(true).value_parser(clap::value_parser!(i32)))
                )
        )
        .subcommand(
            Command::new("digest-send")
                .about("Send an email with the newest crates")
                .arg(Arg::new("to").required(true))
                .arg(Arg::new("hours_since").required(true).value_parser(clap::value_parser!(i32)))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => cr8s::commands::create_user(
                // unwrap and convert reference to value -> to_owned()
                sub_matches.get_one::<String>("username").unwrap().to_owned(),
                sub_matches.get_one::<String>("password").unwrap().to_owned(),
                sub_matches.get_many::<String>("roles").unwrap().map(|v| v.to_string()).collect(),
            ),
            Some(("list", _)) => cr8s::commands::list_users(),
            Some(("delete", sub_matches)) => cr8s::commands::delete_user(
                sub_matches.get_one::<i32>("id").unwrap().to_owned()
            ),
            _ => {}
        },
        Some(("digest-send", sub_matches)) => cr8s::commands::send_digest(
            sub_matches.get_one::<String>("to".trim()).unwrap().to_owned(),
            sub_matches.get_one::<i32>("hours_since").unwrap().to_owned(),
        ),
        _ => {}
    }
}