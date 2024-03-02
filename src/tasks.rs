use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::path::PathBuf,
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    /*
     * Instructs the `ts_seconds` module to serialize (or deserialize vice versa) a value of type `DateTime<Utc>`
     * as a UNIX epoch (seconds since January 1, 1970).
     * We don't define the created_at field by using the `DateTime<Local>` type,
     * this means that the` chrono::serde::ts_seconds` module expects `DateTime` structs to be specialized over the Utc type.
     */
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

/*
 * `Display` traits are used to display type values in a user-readable format.
 * In the `Display::fmt` function, we convert the `DateTime<Utc>` timestamp into a `DateTime<Local>` struct,
 * so users can see the date and time the task was created in local time.
 * Declares that `std::fmt::Display` trait is implemented for the Task structure.
 */
impl fmt::Display for Task {
    /*
     * The `<'_>` relates to the lifetime specifier, specifically the notion of an "optional lifetime".
     * In Rust, the lifetime is used to tell the compiler how long the reference is valid.
     * This allows memory safety to be guaranteed at compile time.
     * By using '_' instead of writing the lifetime annotation explicitly,
     * you can indicate that "this lifetime should be inferred by the compiler".
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        /*
         * `{:<50}`: a left-aligned string padded with 50 spaces.
         * `[{}]`: the date and time the task was created, inside brackets.
         */
        write!(f, "{:<50} [{}]", self.text, created_at);
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    /*
     * Rewind the file after reading from it.
     * `file.seek(...)` is used to move the cursor to a specific position in the file.
     * `SeekFrom::Start(0)` points to the 0th byte from the beginning of the file, that is, the start position of the file.
     * In other words, this means that the cursor is set to the first position in the file.
     * Because we moved the cursor to the end of the file, we need to rewind the file before we write over it again.
     * If we don't rewind the file, we'd begin writing at the cursor's last position, which would cause a malformed JSON file.
     */
    file.seek(SeekFrom::Start(0))?;

    /*
     * Consume the file's contents as a vector of tasks.
     * `match serde_json::from_reader(&file)` reads the contents of a file
     * and attempts to convert (deserialize) it into a vector of type Task.
     * Here the `serde_json::from_reader(&file)` function takes a file pointer `&file` as an argument
     * and attempts to convert the JSON-format content of the file into a Rust data structure.
     */
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,

        /*
         * If an error occurs but the error indicates that the end of the file (EOF: End Of File) has been reached,
         * an empty vector (`Vec::new()`) is initialized as tasks.
         * This is used when the file is empty or the correct data has not yet been written.
         */
        Err(e) if e.is_eof => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks);
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    /*
     * Open the file.
     * The question mark symbol (`?`) after that statement is used to propagate errors without writing too much boilerplate code.
     * It's syntax sugar for early returning an error if that error matches with the return type of the function it's in.
     * So below snippets are equivalent:
     * fn function_1() -> Result(Success, Failure) {
     *      match operation_that_might_fail() {
     *          Ok(success) => success,
     *          Err(failure) => return Err(failure),
     *      }
     *  }
     *
     *  fn function_2() -> Result(Success, Failure) {
     *      operation_that_might_fail()?
     *  }
     *
     * See doc: https://doc.rust-lang.org/reference/expressions/operator-expr.html#:~:text=The%20question%20mark%20operator%20(%20%3F%20),%3E%20type%2C%20it%20propagates%20errors.
     */
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    /*
     * Write the modified task list back into the file.
     */
    tasks.push(task);
    serde_json::to_write(file, &tasks)?;

    /*
     * `()` is called `unit`.
     * If no return type is specified for the function, it returns an empty tuple(`()`).
     */
    Ok(());
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    /*
     * Write the modified task list back into the file.
     * `file.set_len(0)?` sets the size of the file to 0 bytes. This completely deletes the existing contents of the file.
     * This procedure is used to empty a file before completely replacing its contents with new data.
     */
    file.set_len(0)?;
    serde_json::to_write(file, &tasks)?;

    Ok(());
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(());
}
