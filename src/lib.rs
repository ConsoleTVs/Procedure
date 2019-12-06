extern crate yansi;

use std::fmt::Display;
use std::io::{stdout, Write};
use std::ops::FnOnce;
use yansi::Paint;

/// Determines the default padding
const DEFAULT_PADDING: usize = 12;

/// Representation of the progress structure.
pub struct Progress<'a> {
    /// Used to store the prefix action.
    action: &'a str,
    /// Used to store the description of the action.
    description: &'a str,
    /// Used to store the padding of the action.
    padding: usize,
    /// Stores the current percentage value (from 0-100).
    value: u8,
}

/// Implementation of the progress structure.
impl<'a> Progress<'a> {
    /// Creates a new progress struct.
    pub fn new(action: &'a str, description: &'a str, padding: usize) -> Progress<'a> {
        Progress {
            action,
            description,
            padding,
            value: 0,
        }
    }

    /// Initializes the progress by setting a percentage of 0%.
    pub fn initialize(&mut self) {
        self.set_from(0, 100, 0);
    }

    /// Sets the percentage value from 1 to 100 of the progress.
    pub fn set(&mut self, value: usize) {
        self.set_from(0, 100, value)
    }

    /// Increment the percentage value with the given offset.
    pub fn increment(&mut self, offset: usize) {
        self.set(self.value as usize + offset);
    }

    /// Sets the percentage value automatically given a min, a max and the current value.
    /// Please note that min < value < max.
    pub fn set_from(&mut self, min: usize, max: usize, value: usize) {
        self.value = (((value - min) * 100) / (max - min)) as u8;
        print!(
            "\r{:>3$} [{:>3}%] {:<4$}",
            Paint::yellow(self.action),
            self.value,
            self.description,
            self.padding,
            self.description.len()
        );
        stdout().flush().unwrap_or_default();
    }

    /// Finishes the progress (and sets the percentage to 100%) with the given value.
    /// The value will replace the action description in the stdout.
    fn ok<R>(&mut self, result: &R)
    where
        R: Display,
    {
        self.value = 100;
        println!(
            "\r{:>3$} [{:>3}%] {:<4$}",
            Paint::green(self.action),
            self.value,
            result,
            self.padding,
            self.description.len()
        );
    }

    /// Finishes the progress (does not modify the percentage value) with the given error value.
    /// The value will replace the action description in the stdout.
    fn err<E>(&mut self, error: &E)
    where
        E: Display,
    {
        println!(
            "\r{:>3$} [{:>3}%] {:<4$}",
            Paint::red(self.action),
            self.value,
            error,
            self.padding,
            self.description.len()
        );
    }
}

/// Proceeds the execution of `function` with a nice formatted `action`.
/// The default padding is {DEFAULT_PADDING}.
pub fn proceed<F, R, D, E>(action: &str, description: &str, function: F) -> Result<R, E>
where
    F: FnOnce(&mut Progress) -> Result<(R, D), E>,
    D: Display,
    E: Display,
{
    proceed_padded(action, description, function, DEFAULT_PADDING)
}

/// Proceeds the execution of `function` with a nice formatted `action` given a left `padding`.
pub fn proceed_padded<F, R, D, E>(
    action: &str,
    description: &str,
    function: F,
    padding: usize,
) -> Result<R, E>
where
    F: FnOnce(&mut Progress) -> Result<(R, D), E>,
    D: Display,
    E: Display,
{
    let mut progress = Progress::new(action, description, padding);
    progress.initialize();
    match function(&mut progress) {
        Ok(result) => {
            progress.ok(&result.1);
            Ok(result.0)
        }
        Err(error) => {
            progress.err(&error);
            Err(error)
        }
    }
}

/// Prints a success action with a description.
pub fn success(action: &str, description: &str) {
    success_padded(action, description, DEFAULT_PADDING);
}

/// Prints a success action with a description. Includes an additional padding on the left.
pub fn success_padded(action: &str, description: &str, padding: usize) {
    println!("\r{:>2$} {:}", Paint::green(action), description, padding);
}

/// Prints an error action with a description.
pub fn error(action: &str, description: &str) {
    error_padded(action, description, DEFAULT_PADDING);
}

/// Prints an error action with a description. Includes an additional padding on the left.
pub fn error_padded(action: &str, description: &str, padding: usize) {
    println!("\r{:>2$} {:}", Paint::red(action), description, padding);
}

/// Prints a warning action with a description.
pub fn warning(action: &str, description: &str) {
    warning_padded(action, description, DEFAULT_PADDING);
}

/// Prints a warning action with a description. Includes an additional padding on the left.
pub fn warning_padded(action: &str, description: &str, padding: usize) {
    println!("\r{:>2$} {:}", Paint::yellow(action), description, padding);
}

/// Prints an info action with a description.
pub fn info(action: &str, description: &str) {
    info_padded(action, description, DEFAULT_PADDING);
}

/// Prints an info action with a description. Includes an additional padding on the left.
pub fn info_padded(action: &str, description: &str, padding: usize) {
    println!("\r{:>2$} {:}", Paint::cyan(action), description, padding);
}
