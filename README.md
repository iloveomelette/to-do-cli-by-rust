# Rust Training

## Introduction

The application is a command-line to-do tracker.

It records our tasks into a text file, displays them as a list in the terminal, and lets us mark them complete.

## Usage

**NOTE**: The following instructions assume that you have Docker installed on your machine.

### Step 1: Pull the Docker Image

```bash
$ docker pull rust:1.76.0-bookworm
```

### Step 2: Run the Docker Container

```bash
$ bin/run -- -h
Rusty Journal 0.1.0
A command line to-do app written in Rust.

USAGE:
    todo [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-file <journal-file>    Use a different journal file

SUBCOMMANDS:
    add     Write tasks to the journal file
    done    Remove an entry from the journal file by position
    help    Prints this message or the help of the given subcommand(s)
    list    List all tasks in the journal file
```

**NOTE**: If you can't run the `bin/run` script, you may need to give it execute permissions.

```bash
$ chmod +x bin/run
```

### Example

```bash
$ bin/run -- -j test-journal.json add "Study Rust"
```

```bash
$ bin/run -- -j test-journal.json list
1: buy milk                                           [2024-03-02 11:27](UTC)
2: water the plants                                   [2024-03-02 11:28](UTC)
3: Study Rust                                         [2024-03-03 02:17](UTC)
```

## Thanks

[Microsoft Training - Build a command-line to-do list program](https://learn.microsoft.com/en-us/training/modules/rust-create-command-line-program/1-introduction?ns-enrollment-type=learningpath&ns-enrollment-id=learn.languages.rust-first-steps)
