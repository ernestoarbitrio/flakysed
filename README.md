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

To process a txt file that contains a complete report of a failing `pytest` pipeline with a complete traceback like:

```
[gw0] [36m [ 98%]  [0m [32mPASSED [0m lib/tests/unit/test_a.py::TestA::test_a
[gw7] [36m [ 98%]  [0m [32mPASSED [0m lib/tests/unit/test_b.py::TestA::test_b
[gw1] [36m [ 98%]  [0m [32mPASSED [0m lib/tests/unit/test_c.py::TestA::test_c
[gw7] [36m [ 98%]  [0m [32mPASSED [0m lib/tests/unit/test_d.py::TestA::test_d
[gw7] [36m [ 98%]  [0m [32mPASSED [0m lib/tests/unit/test_d.py::TestA::test_d1
[gw7] [36m [ 98%]  [0m [32mFAILED [0m lib/tests/unit/test_d.py::TestA::test_d2

=================================== FAILURES ===================================
[31m [1m______________________ test_d2 _______________________ [0m
[gw3] linux -- Python 3.11.11 /home/app/venv/bin/python
...
lib/src/cr/lib/importer/a.py                                                          362     44  87.85%   12-15, 46, 52, 55, 58, 
lib/src/cr/lib/importer/p.py                                                          155     69  55.48%   60-62, 66-67, 78-81, 
lib/src/cr/lib/importer/s.py                                                          74       1  98.65%   112
```

and after this command

```bash
process_file --input tests_failure.txt --output tests_faliure_cleaned.txt --worker gw7
```

the cleaned files will contains:

```
lib/tests/unit/test_b.py::TestA::test_b
lib/tests/unit/test_d.py::TestA::test_d
lib/tests/unit/test_d.py::TestA::test_d1
lib/tests/unit/test_d.py::TestA::test_d2
```

This command will:

1. Read `tests_failure.txt`.
2. Filter logs related to worker `gw7`.
3. Stop processing at the first test failure (if present).
4. Write the cleaned and normalized logs to `tests_faliure_cleaned.txt`.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

For questions or issues, please open an issue in the repository.
