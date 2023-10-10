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
                        // Specify/provide a value_delimeter a.k.a value seperator
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                )
                .subcommand(
                    Command::new("list")
                        .about("List all available users")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by ID")
                        .arg(Arg::new("id").required(true))
                )
        ).get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matcheds)) => create_user(),
            Some(("list", sub_matcheds)) => list_users(),
            Some(("delete", sub_matcheds)) => delete_user(),
        },
        _ => {},
    }
}