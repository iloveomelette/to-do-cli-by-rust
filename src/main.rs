/*
 * Organize code into modules.
 * Use mod to create new modules to encapsulate code, including other modules
 *
 * doc: https://doc.rust-lang.org/std/keyword.mod.html
 */
mod cli;
use structopt::StructOpt;

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
