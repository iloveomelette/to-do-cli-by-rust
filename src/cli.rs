use std::path::PathBuf;
use structopt::StructOpt;

/*
 * Automatically adds the ability to debug output
 * and command line analysis with structopt for `Action` enum.
 * Use `structopt` to create an instance of this type directly from command line arguments.
 */
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

/*
 * `name` and `about` is displayed when a user executes a help command.
 */
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust."
)]
pub struct CommandLineArgs {
    /*
     * Some applications, especially large ones, split their functionality through the use of “subcommands”.
     * Each of these act somewhat like a separate command, but is part of the larger group.
     * One example is `git`, which has subcommands such as `add`, `commit`, and `clone`, to mention just a few.
     * clap has this functionality, and structopt supports it through enums
     */
    #[structopt(subcommand)]
    pub action: Action,

    // short and long flags (-d, --debug) will be deduced from the field's name
    // the long option will be translated by default to kebab case,
    // i.e. `--journal-file`.
    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
