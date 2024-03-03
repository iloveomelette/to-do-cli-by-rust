/*
 * Organize code into modules.
 * Use mod to create new modules to encapsulate code, including other modules
 *
 * doc: https://doc.rust-lang.org/std/keyword.mod.html
 */
mod cli;
mod tasks;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;
use anyhow::anyhow;

/*
 * Call the `home::home_dir()` function to obtain the path to the current user's home directory.
 * The function returns an `Option<PathBuf>` type.
 * `Some(PathBuf)` is returned if the home directory is found, `None` if not found.
 *
 * If the path to the home directory is found (`Some(PathBuf)`),
 * the `.map` method is used on that path to add the file name `.rusty-journal.json` to the path.
 * The `.map` method performs processing only if Option is `Some`, and does nothing if Option is `None`.
 */
fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    /*
     * Get the command-line arguments.
     */
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    /*
     * Unpack the journal file.
     */
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}
