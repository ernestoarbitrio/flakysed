use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};

/// Struct to process files by filtering lines based on worker regex pattern
/// and performing replacements to generate a simplified output.
///
/// # Panics
/// This function may panic if:
/// - The regex compilation fails (due to invalid patterns), which results in `unwrap()` calls in `Regex::new` or `FancyRegex::new`.
/// - A capture group is missing when processing the line, which triggers the `unwrap()` in `captures.get(1)`.
pub struct ProcessFile {}

impl ProcessFile {
    pub fn run(input_file: &str, output_file: &str, worker: &str) -> io::Result<()> {
        // Reads the content of the input file, filters lines by worker, processes them,
        // and writes the result to the output file.
        //
        // # Panics
        // This function may panic if:
        // - The regex compilation fails for any of the patterns (as they use `unwrap()`).
        // - The capture group is missing in the line being processed, which will cause an `unwrap()` panic in `captures.get(1)`.
        let content: String = fs::read_to_string(input_file)?.replace('\x1b', "");

        let grep_pattern: Regex = Regex::new(&format!(r"\[{}\]", worker)).unwrap();
        let tests_regexp: Regex = Regex::new(r".*?/tests/").unwrap();
        let brackets_regexp: Regex = Regex::new(r"\[[^\]]*\]\s*").unwrap();
        let test_ref_regexp: FancyRegex = FancyRegex::new(r"(\S+)(?=\/tests\/)").unwrap();

        let filtered_lines: Vec<&str> = content
            .lines()
            .filter(|line: &&str| grep_pattern.is_match(line))
            .collect::<Vec<&str>>();

        let fgrep_lines: Vec<&&str> = filtered_lines
            .iter()
            .filter(|&&line| line.contains("tests/"))
            .collect::<Vec<_>>();

        let failed_index: Option<usize> =
            fgrep_lines.iter().position(|&line| line.contains("FAILED"));
        let fgrep_lines: &[&&str] = match failed_index {
            Some(index) => &fgrep_lines[..=index],
            None => &fgrep_lines[..],
        };

        let mut output: File = File::create(output_file)?;

        for &line in fgrep_lines {
            let newline: String = line.replace('\r', "");

            match test_ref_regexp.captures(&newline) {
                Ok(Some(captures)) => {
                    let group: &str = captures
                        .get(1)
                        .ok_or_else(|| {
                            io::Error::new(io::ErrorKind::InvalidData, "No capture group found")
                        })?
                        .as_str();

                    let mut modified_newline = tests_regexp.replace(&newline, "tests/").to_string();
                    modified_newline = brackets_regexp.replace(&modified_newline, "").to_string();

                    writeln!(output, "{}/{}", group, modified_newline)?;
                }
                Ok(None) => {
                    eprintln!("No match found for line: {}", newline);
                    continue;
                }
                Err(err) => {
                    eprintln!("Error running regex: {}", err);
                    continue;
                }
            }
        }

        Ok(())
    }
}
