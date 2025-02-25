/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::cmp::max;

use colored::*;
use hurl_core::ast::SourceInfo;
use hurl_core::error::Error;

use crate::runner::{HurlResult, Value};

/// A simple logger to log app related event (start, high levels error, etc...).
/// When we run an [`hurl_core::ast::HurlFile`], user has to provide a dedicated Hurl logger (see [`Logger`]).
pub struct BaseLogger {
    pub color: bool,
    pub verbose: bool,
}

impl BaseLogger {
    pub fn new(color: bool, verbose: bool) -> BaseLogger {
        BaseLogger { color, verbose }
    }

    pub fn info(&self, message: &str) {
        log_info(message);
    }

    pub fn debug(&self, message: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug(message);
        } else {
            log_debug_no_color(message);
        }
    }

    pub fn warning(&self, message: &str) {
        if self.color {
            log_warning(message);
        } else {
            log_warning_no_color(message);
        }
    }

    pub fn error(&self, message: &str) {
        if self.color {
            log_error(message);
        } else {
            log_error_no_color(message);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorFormat {
    Short,
    Long,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    VeryVerbose,
}

impl Verbosity {
    pub fn from(verbose: bool, very_verbose: bool) -> Option<Verbosity> {
        match (verbose, very_verbose) {
            (_, true) => Some(Verbosity::VeryVerbose),
            (true, false) => Some(Verbosity::Verbose),
            _ => None,
        }
    }
}

/// A Hurl dedicated logger for an Hurl file. Contrary to [`BaseLogger`], this logger can display
/// rich error for parsing and runtime errors.
#[derive(Clone)]
pub struct Logger {
    pub(crate) color: bool,
    pub(crate) error_format: ErrorFormat,
    pub(crate) filename: String,
    pub(crate) progress_bar: bool,
    pub(crate) test: bool,
    pub(crate) verbosity: Option<Verbosity>,
}

impl From<&LoggerOptions> for Logger {
    fn from(options: &LoggerOptions) -> Self {
        Logger {
            color: options.color,
            error_format: options.error_format,
            filename: options.filename.clone(),
            progress_bar: options.progress_bar,
            test: options.test,
            verbosity: options.verbosity,
        }
    }
}

pub struct LoggerOptions {
    pub(crate) color: bool,
    pub(crate) error_format: ErrorFormat,
    pub(crate) filename: String,
    pub(crate) progress_bar: bool,
    pub(crate) test: bool,
    pub(crate) verbosity: Option<Verbosity>,
}

pub struct LoggerOptionsBuilder {
    color: bool,
    error_format: ErrorFormat,
    filename: String,
    progress_bar: bool,
    test: bool,
    verbosity: Option<Verbosity>,
}

impl LoggerOptionsBuilder {
    /// Returns a new Logger builder with a default values.
    pub fn new() -> Self {
        LoggerOptionsBuilder::default()
    }

    /// Sets color usage.
    pub fn color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }

    /// Control the format of error messages.
    /// If `error_format` is [`ErrorFormat::Long`], the HTTP request and response that has
    /// errors is displayed (headers, body, etc..)
    pub fn error_format(&mut self, error_format: ErrorFormat) -> &mut Self {
        self.error_format = error_format;
        self
    }

    /// Sets verbose logger.
    pub fn verbosity(&mut self, verbosity: Option<Verbosity>) -> &mut Self {
        self.verbosity = verbosity;
        self
    }

    /// Sets progress bar.
    pub fn progress_bar(&mut self, progress_bar: bool) -> &mut Self {
        self.progress_bar = progress_bar;
        self
    }

    /// Sets test.
    pub fn test(&mut self, test: bool) -> &mut Self {
        self.test = test;
        self
    }

    /// Sets filename.
    pub fn filename(&mut self, filename: &str) -> &mut Self {
        self.filename = filename.to_string();
        self
    }

    /// Creates a new logger.
    pub fn build(&self) -> LoggerOptions {
        LoggerOptions {
            color: self.color,
            error_format: self.error_format,
            filename: self.filename.clone(),
            progress_bar: self.progress_bar,
            test: self.test,
            verbosity: self.verbosity,
        }
    }
}

impl Default for LoggerOptionsBuilder {
    fn default() -> Self {
        LoggerOptionsBuilder {
            color: false,
            error_format: ErrorFormat::Short,
            filename: String::new(),
            progress_bar: false,
            test: false,
            verbosity: None,
        }
    }
}

impl Logger {
    pub fn info(&self, message: &str) {
        log_info(message);
    }

    pub fn debug(&self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug(message);
        } else {
            log_debug_no_color(message);
        }
    }

    pub fn debug_curl(&self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_curl(message);
        } else {
            log_debug_curl_no_color(message);
        }
    }

    pub fn debug_error<E: Error>(&self, content: &str, error: &E, entry_src_info: SourceInfo) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_error(&self.filename, content, error, entry_src_info);
        } else {
            log_debug_error_no_color(&self.filename, content, error, entry_src_info);
        }
    }

    pub fn debug_header_in(&self, name: &str, value: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_header_in(name, value);
        } else {
            log_debug_header_in_no_color(name, value);
        }
    }

    pub fn debug_header_out(&self, name: &str, value: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_header_out(name, value);
        } else {
            log_debug_header_out_no_color(name, value);
        }
    }

    pub fn debug_important(&self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_important(message);
        } else {
            log_debug_no_color(message);
        }
    }

    pub fn debug_status_version_in(&self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_status_version_in(line);
        } else {
            log_debug_status_version_in_no_color(line);
        }
    }

    pub fn warning(&self, message: &str) {
        if self.color {
            log_warning(message);
        } else {
            log_warning_no_color(message);
        }
    }

    pub fn error(&self, message: &str) {
        if self.color {
            log_error(message);
        } else {
            log_error_no_color(message);
        }
    }

    pub fn error_parsing_rich<E: Error>(&self, content: &str, error: &E) {
        if self.color {
            log_error_rich(&self.filename, content, error, None);
        } else {
            log_error_rich_no_color(&self.filename, content, error, None);
        }
    }

    pub fn error_runtime_rich<E: Error>(
        &self,
        content: &str,
        error: &E,
        entry_src_info: SourceInfo,
    ) {
        if self.color {
            log_error_rich(&self.filename, content, error, Some(entry_src_info));
        } else {
            log_error_rich_no_color(&self.filename, content, error, Some(entry_src_info));
        }
    }

    pub fn debug_method_version_out(&self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_debug_method_version_out(line);
        } else {
            log_debug_method_version_out_no_color(line);
        }
    }

    pub fn capture(&self, name: &str, value: &Value) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            log_capture(name, value);
        } else {
            log_capture_no_color(name, value);
        }
    }

    pub fn test_running(&self, current: usize, total: usize) {
        if !self.test {
            return;
        }
        if self.color {
            log_test_running(&self.filename, current, total);
        } else {
            log_test_running_no_color(&self.filename, current, total);
        }
    }

    pub fn test_progress(&self, entry_index: usize, count: usize) {
        if !self.progress_bar {
            return;
        }
        log_test_progress(entry_index, count);
    }

    pub fn test_completed(&self, result: &HurlResult) {
        if !self.test {
            return;
        }
        if self.color {
            log_test_completed(result, &self.filename);
        } else {
            log_test_completed_no_color(result, &self.filename);
        }
    }

    pub fn test_erase_line(&self) {
        if !self.progress_bar {
            return;
        }
        // This is the "EL - Erase in Line" sequence. It clears from the cursor
        // to the end of line.
        // https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_sequences
        eprint!("\x1B[K");
    }
}

fn log_info(message: &str) {
    eprintln!("{message}");
}

fn log_debug(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "*".blue().bold());
    } else {
        eprintln!("{} {}", "*".blue().bold(), message);
    }
}

fn log_debug_no_color(message: &str) {
    if message.is_empty() {
        eprintln!("*");
    } else {
        eprintln!("* {message}");
    }
}

fn log_debug_curl(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "**".blue().bold());
    } else {
        eprintln!("{} {}", "**".blue().bold(), message.green());
    }
}

fn log_debug_curl_no_color(message: &str) {
    if message.is_empty() {
        eprintln!("**");
    } else {
        eprintln!("** {message}");
    }
}

fn log_debug_important(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "*".blue().bold());
    } else {
        eprintln!("{} {}", "*".blue().bold(), message.bold());
    }
}

fn log_debug_error<E: Error>(filename: &str, content: &str, error: &E, entry_src_info: SourceInfo) {
    let message = error_string(filename, content, error, Some(entry_src_info), true);
    split_lines(&message).iter().for_each(|l| log_debug(l));
}

fn log_debug_error_no_color<E: Error>(
    filename: &str,
    content: &str,
    error: &E,
    entry_src_info: SourceInfo,
) {
    let message = error_string(filename, content, error, Some(entry_src_info), false);
    split_lines(&message)
        .iter()
        .for_each(|l| log_debug_no_color(l));
}

fn log_debug_header_in(name: &str, value: &str) {
    eprintln!("< {}: {}", name.cyan().bold(), value);
}

fn log_debug_header_in_no_color(name: &str, value: &str) {
    eprintln!("< {name}: {value}");
}

fn log_debug_header_out(name: &str, value: &str) {
    eprintln!("> {}: {}", name.cyan().bold(), value);
}

fn log_debug_header_out_no_color(name: &str, value: &str) {
    eprintln!("> {name}: {value}");
}

fn log_debug_method_version_out(line: &str) {
    eprintln!("> {}", line.purple().bold());
}

fn log_debug_method_version_out_no_color(line: &str) {
    eprintln!("> {line}");
}

fn log_debug_status_version_in(line: &str) {
    eprintln!("< {}", line.green().bold());
}

fn log_debug_status_version_in_no_color(line: &str) {
    eprintln!("< {line}");
}

fn log_warning(message: &str) {
    eprintln!("{}: {}", "warning".yellow().bold(), message.bold());
}

fn log_warning_no_color(message: &str) {
    eprintln!("warning: {message}");
}

fn log_error(message: &str) {
    eprintln!("{}: {}", "error".red().bold(), message.bold());
}

fn log_error_no_color(message: &str) {
    eprintln!("error: {message}");
}

fn log_error_rich<E: Error>(
    filename: &str,
    content: &str,
    error: &E,
    entry_src_info: Option<SourceInfo>,
) {
    let message = error_string(filename, content, error, entry_src_info, true);
    eprintln!("{}: {}\n", "error".red().bold(), &message);
}

fn log_error_rich_no_color<E: Error>(
    filename: &str,
    content: &str,
    error: &E,
    entry_src_info: Option<SourceInfo>,
) {
    let message = error_string(filename, content, error, entry_src_info, false);
    eprintln!("error: {}\n", &message);
}

fn log_capture(name: &str, value: &Value) {
    eprintln!("{} {}: {}", "*".blue().bold(), name.yellow().bold(), value);
}

fn log_capture_no_color(name: &str, value: &Value) {
    eprintln!("* {name}: {value}");
}

fn log_test_running(filename: &str, current: usize, total: usize) {
    eprintln!(
        "{}: {} [{}/{}]",
        filename.bold(),
        "Running".cyan().bold(),
        current,
        total
    );
}

fn log_test_running_no_color(filename: &str, current: usize, total: usize) {
    eprintln!("{filename}: Running [{current}/{total}]");
}

fn log_test_progress(entry_index: usize, count: usize) {
    let progress = progress_string(entry_index, count);
    eprint!(" {progress}\r");
}

/// Returns the progress string with the current entry at `entry_index`.
fn progress_string(entry_index: usize, count: usize) -> String {
    const WIDTH: usize = 24;
    // We report the number of entries already processed.
    let progress = (entry_index - 1) as f64 / count as f64;
    let col = (progress * WIDTH as f64) as usize;
    let completed = if col > 0 {
        "=".repeat(col)
    } else {
        String::new()
    };
    let void = " ".repeat(WIDTH - col - 1);
    format!("[{completed}>{void}] {entry_index}/{count}")
}

fn log_test_completed(result: &HurlResult, filename: &str) {
    let state = if result.success {
        "Success".green().bold()
    } else {
        "Failure".red().bold()
    };
    let count = result.entries.iter().flat_map(|r| &r.calls).count();
    eprintln!(
        "{}: {} ({} request(s) in {} ms)",
        filename.bold(),
        state,
        count,
        result.time_in_ms
    );
}

fn log_test_completed_no_color(result: &HurlResult, filename: &str) {
    let state = if result.success { "Success" } else { "Failure" };
    let count = result.entries.iter().flat_map(|r| &r.calls).count();
    eprintln!(
        "{}: {} ({} request(s) in {} ms)",
        filename, state, count, result.time_in_ms
    );
}

/// Returns the string representation of an `error`, given `lines` of content and a `filename`.
///
/// The source information where the error occurred can be retrieved in `error`; optionally,
/// `entry_src_info` is the optional source information for the entry where the error happened.
/// If `colored` is true, the string use ANSI escape codes to add color and improve the readability
/// of the representation.
///
/// Example:
///
/// ```text
/// Assert status code
///  --> test.hurl:2:10
///   |
/// 2 | HTTP/1.0 200
///   |          ^^^ actual value is <404>
///   |
/// ```
pub(crate) fn error_string<E: Error>(
    filename: &str,
    content: &str,
    error: &E,
    entry_src_info: Option<SourceInfo>,
    colored: bool,
) -> String {
    let mut text = String::new();
    let lines = split_lines(content);
    let entry_line = entry_src_info.map(|e| e.start.line);
    let error_line = error.source_info().start.line;
    let error_column = error.source_info().start.column;
    // The number of digits of the lines count.
    let loc_max_width = max(lines.len().to_string().len(), 2);
    let separator = "|";
    let separator = if colored {
        separator.blue().bold().to_string()
    } else {
        separator.to_string()
    };
    let spaces = " ".repeat(loc_max_width);
    let prefix = format!("{spaces} {separator}");

    // 1. First line is the description, ex. `Assert status code`.
    let description = if colored {
        error.description().bold().to_string()
    } else {
        error.description()
    };
    text.push_str(&description);
    text.push('\n');

    // 2. Second line is the filename info, ex. ` --> test.hurl:2:10`
    let arrow = "-->";
    let arrow = if colored {
        arrow.blue().bold().to_string()
    } else {
        arrow.to_string()
    };
    let line = format!("{spaces}{arrow} {filename}:{error_line}:{error_column}");
    text.push_str(&line);
    text.push('\n');

    // 3. Appends line separator.
    text.push_str(&prefix);
    text.push('\n');

    // 4. Appends the optional entry line.
    if let Some(entry_line) = entry_line {
        if entry_line != error_line {
            let line = lines.get(entry_line - 1).unwrap();
            let line = if colored {
                line.bright_black().to_string()
            } else {
                line.to_string()
            };
            text.push_str(&prefix);
            text.push(' ');
            text.push_str(&line);
            text.push('\n');
        }
        if error_line - entry_line > 1 {
            text.push_str(&prefix);
            let dots = " ...\n";
            let dots = if colored {
                dots.bright_black().to_string()
            } else {
                dots.to_string()
            };
            text.push_str(&dots);
        }
    }

    // 5. Then, we build error line (whitespace is uniformized)
    // ex. ` 2 | HTTP/1.0 200`
    let line = line_with_loc(&lines, error_line, &separator, colored);
    text.push_str(&line);
    text.push('\n');

    // 6. Then, we append the error detailed message:
    // ```
    // |   actual:   byte array <ff>
    // |   expected: byte array <00>
    // ````

    // Explicit asserts output is multi-line, with actual and expected value aligned, while
    // other errors (implicit assert for instance) are one-line, with a column error marker "^^^..."
    // on a second line.
    // So, we have "marked" explicit asserts to suppress the display of the column error marker
    // by setting their `source_info`'s column to 0 (see [`hurl::runner::predicate::eval_predicate::`]).
    let message = if error_column == 0 {
        let new_prefix = format!("{prefix}   "); // actual and expected are prefixed by 2 spaces
        let fix_me = error.fixme();
        add_line_prefix(&fix_me, &new_prefix, colored)
    } else {
        // We take tabs into account because we have normalize the display of the error line by replacing
        // tabs with 4 spaces.
        // TODO: add a unit test with tabs in source info.
        let mut tab_shift = 0;
        let line_raw = lines.get(error_line - 1).unwrap();
        for (i, c) in line_raw.chars().enumerate() {
            if i >= error_column - 1 {
                break;
            };
            if c == '\t' {
                tab_shift += 1;
            }
        }

        // Error source info start and end can be on different lines, we insure a minimum width.
        let width = if error.source_info().end.column > error_column {
            error.source_info().end.column - error_column
        } else {
            1
        };

        let mut fix_me = "^".repeat(width);
        fix_me.push(' ');
        fix_me.push_str(&error.fixme());
        if colored {
            fix_me = fix_me.red().bold().to_string();
        }
        format!(
            "{spaces} {separator} {}{fix_me}",
            " ".repeat(error_column - 1 + tab_shift * 3)
        )
    };
    text.push_str(&message);
    text.push('\n');

    // 6. Appends final line separator.
    text.push_str(&prefix);

    text
}

/// Returns the `line` count prefix.
/// Example: `   45 `
fn line_with_loc(lines: &[&str], loc: usize, separator: &str, colored: bool) -> String {
    let mut text = String::new();
    let loc_max_width = max(lines.len().to_string().len(), 2);
    let line = lines.get(loc - 1).unwrap();
    let line = line.replace('\t', "    ");
    let mut line_number = format!("{loc:>loc_max_width$}");
    if colored {
        line_number = line_number.blue().bold().to_string();
    }
    text.push_str(&line_number);
    text.push(' ');
    text.push_str(separator);
    if !line.is_empty() {
        text.push(' ');
        text.push_str(&line);
    }
    text
}

/// Prefixes each line of the string `s` with a `prefix` and returns the new string.
/// If `colored` is true, each line is colored with ANSI escape codes.
fn add_line_prefix(s: &str, prefix: &str, colored: bool) -> String {
    split_lines(s)
        .iter()
        .map(|line| {
            if colored {
                format!("{}{}", prefix, line.red().bold())
            } else {
                format!("{prefix}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Splits this `text` to a list of LF/CRLF separated lines.
fn split_lines(text: &str) -> Vec<&str> {
    regex::Regex::new(r"\n|\r\n").unwrap().split(text).collect()
}

#[cfg(test)]
pub mod tests {
    use hurl_core::ast::{Pos, SourceInfo};

    use super::*;
    use crate::runner;

    #[test]
    fn test_add_line_prefix_no_colored() {
        assert_eq!(
            add_line_prefix("line1\nline2\nline3", ">", false),
            ">line1\n>line2\n>line3"
        );
    }

    #[test]
    fn test_error_timeout() {
        let content = "GET http://unknown";
        let filename = "test.hurl";
        let inner =
            runner::RunnerError::HttpConnection("(6) Could not resolve host: unknown".to_string());
        let error_source_info = SourceInfo::new(Pos::new(1, 5), Pos::new(1, 19));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19));
        let error = runner::Error::new(error_source_info, inner, true);
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"HTTP connection
  --> test.hurl:1:5
   |
 1 | GET http://unknown
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown
   |"#
        );
    }

    #[test]
    fn test_assert_error_status() {
        let content = r#"GET http://unknown
HTTP/1.0 200
"#;
        let filename = "test.hurl";
        let inner = runner::RunnerError::AssertStatus {
            actual: "404".to_string(),
        };
        let error_source_info = SourceInfo::new(Pos::new(2, 10), Pos::new(2, 13));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18));
        let error = runner::Error::new(error_source_info, inner, true);
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert status code
  --> test.hurl:2:10
   |
   | GET http://unknown
 2 | HTTP/1.0 200
   |          ^^^ actual value is <404>
   |"#
        );
    }

    #[test]
    fn test_invalid_xpath_expression() {
        let content = r#"GET http://example.com
HTTP/1.0 200
[Asserts]
xpath "strong(//head/title)" == "Hello"
"#;
        let filename = "test.hurl";
        let error_source_info = SourceInfo::new(Pos::new(4, 7), Pos::new(4, 29));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 22));
        let error = runner::Error::new(
            error_source_info,
            runner::RunnerError::QueryInvalidXpathEval,
            true,
        );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Invalid XPath expression
  --> test.hurl:4:7
   |
   | GET http://example.com
   | ...
 4 | xpath "strong(//head/title)" == "Hello"
   |       ^^^^^^^^^^^^^^^^^^^^^^ the XPath expression is not valid
   |"#
        );
    }

    #[test]
    fn test_assert_error_jsonpath() {
        let content = r#"GET http://api
HTTP/1.0 200
[Asserts]
jsonpath "$.count" >= 5
"#;
        let filename = "test.hurl";
        let error_source_info = SourceInfo::new(Pos::new(4, 0), Pos::new(4, 0));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14));
        let error = runner::Error {
            source_info: error_source_info,
            inner: runner::RunnerError::AssertFailure {
                actual: "int <2>".to_string(),
                expected: "greater than int <5>".to_string(),
                type_mismatch: false,
            },
            assert: true,
        };
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert failure
  --> test.hurl:4:0
   |
   | GET http://api
   | ...
 4 | jsonpath "$.count" >= 5
   |   actual:   int <2>
   |   expected: greater than int <5>
   |"#
        );
    }

    #[test]
    fn test_assert_error_newline() {
        let content = r#"GET http://localhost
HTTP/1.0 200
```<p>Hello</p>
```
"#;
        let filename = "test.hurl";
        let inner = runner::RunnerError::AssertBodyValueError {
            actual: "<p>Hello</p>\n\n".to_string(),
            expected: "<p>Hello</p>\n".to_string(),
        };
        let error_source_info = SourceInfo::new(Pos::new(3, 4), Pos::new(4, 1));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20));
        let error = runner::Error::new(error_source_info, inner, true);
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert body value
  --> test.hurl:3:4
   |
   | GET http://localhost
   | ...
 3 | ```<p>Hello</p>
   |    ^ actual value is <<p>Hello</p>

>
   |"#
        );
    }

    #[test]
    fn test_parsing_error() {
        let content = "GET abc";
        let filename = "test.hurl";
        let error = hurl_core::parser::Error {
            pos: Pos::new(1, 5),
            recoverable: false,
            inner: hurl_core::parser::ParseError::UrlInvalidStart,
        };
        assert_eq!(
            error_string(filename, content, &error, None, false),
            r#"Parsing URL
  --> test.hurl:1:5
   |
 1 | GET abc
   |     ^ expecting http://, https:// or {{
   |"#
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_progress_string() {
        // Progress strings with 20 entries:
        assert_eq!(progress_string(1,  20), "[>                       ] 1/20");
        assert_eq!(progress_string(2,  20), "[=>                      ] 2/20");
        assert_eq!(progress_string(5,  20), "[====>                   ] 5/20");
        assert_eq!(progress_string(10, 20), "[==========>             ] 10/20");
        assert_eq!(progress_string(15, 20), "[================>       ] 15/20");
        assert_eq!(progress_string(20, 20), "[======================> ] 20/20");

        // Progress strings with 3 entries:
        assert_eq!(progress_string(1, 3), "[>                       ] 1/3");
        assert_eq!(progress_string(2, 3), "[========>               ] 2/3");
        assert_eq!(progress_string(3, 3), "[================>       ] 3/3");

        // Progress strings with 1 entries:
        assert_eq!(progress_string(1, 1), "[>                       ] 1/1");
    }
}
