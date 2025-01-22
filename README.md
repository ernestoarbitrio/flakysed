# FlakySed

This tool processes CircleCI log files, extracting and cleaning log lines associated with a specific worker and stopping at the first test failure (if present). It then writes the cleaned and normalized output to a specified file. The tool is designed to simplify log analysis for debugging and auditing.

## Features

- Filters log lines based on a specific worker identifier.
- Extracts and normalizes paths in the logs, ensuring a consistent format.
- Stops processing at the first test failure for targeted analysis.
- Cleans extraneous data like ANSI escape sequences and unnecessary brackets.
- Outputs the processed results to a user-specified file.

## Requirements

- Rust (for building and running the tool).
- A CircleCI log file to process.

## Installation

`cargo install flakysed`

## Usage

Run the tool from the command line:

```bash
flakysed --input <input_file> --output <output_file> --worker <worker_name>
```

### Arguments

- `--input` (`-i`): Path to the CircleCI log file to process.
- `--output` (`-o`): Path to save the cleaned and processed output.
- `--worker` (`-w`): Worker name to filter logs for processing (e.g., `gw7`).

### Example

To process a log file:

```bash
process_file --input circleci.log --output cleaned_logs.txt --worker gw7
```

This command will:

1. Read `circleci.log`.
2. Filter logs related to worker `gw7`.
3. Stop processing at the first test failure (if present).
4. Write the cleaned and normalized logs to `cleaned_logs.txt`.

### Testing

To run tests (if implemented):

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

For questions or issues, please open an issue in the repository.
