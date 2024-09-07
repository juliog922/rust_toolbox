use clap::{Args, Parser, Subcommand};

/// CLI tool for managing key-value pairs.
///
/// This tool provides basic operations to manipulate key-value pairs
/// such as getting, removing, and setting values associated with keys.
#[derive(Parser)]
#[command(version, about = "A simple key-value store CLI tool", long_about = None)]
struct Cli {
    /// Subcommand to specify the operation (Get, Set, or Rm).
    #[command(subcommand)]
    command: Commands,
}

/// Enum representing the available commands.
///
/// - `Get`: Retrieves the value for a given key.
/// - `Rm`: Removes a key-value pair.
/// - `Set`: Sets the value for a given key.
#[derive(Subcommand)]
enum Commands {
    /// Retrieve the value of a key.
    ///
    /// Example usage: `kvs get <key>`
    Get(Key),

    /// Remove a key-value pair.
    ///
    /// Example usage: `kvs rm <key>`
    Rm(Key),

    /// Set the value for a key.
    ///
    /// Example usage: `kvs set <key> <value>`
    Set(KeyValue),
}

/// Represents a key argument for commands like `Get` and `Rm`.
///
/// This structure is used when you only need to provide a key, such as when retrieving
/// or deleting a key-value pair.
#[derive(Args)]
struct Key {
    /// The key whose value you want to get or remove.
    key_value: String,
}

/// Represents a key-value pair for the `Set` command.
///
/// This structure is used when you want to associate a value with a key.
#[derive(Args)]
struct KeyValue {
    /// The key for which you want to set a value.
    key_value: String,

    /// The value to be associated with the key.
    value: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get(_) => {
            unimplemented!("unimplemented")
        }
        Commands::Rm(_) => {
            unimplemented!("unimplemented")
        }
        Commands::Set(_) => {
            unimplemented!("unimplemented")
        }
    }
}
