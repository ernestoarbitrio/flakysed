use flakysed::ProcessFile;
use std::fs::{self, File};
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_temp_file(content: &str, path: &str) -> io::Result<()> {
        let mut file: File = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn read_temp_file(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn cleanup_temp_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_process_file_success() {
        let input: &str = r#"
        [worker1] PASSED server/tests/something/thing1[ns]
        [worker1] PASSED lib/tests/something/thing1[ns]
        [worker2] PASSED server/tests/something/thing2[ns]
        [worker1] FAILED server/tests/something/thing3[ns]
        [worker1] PASSED server/tests/something/thing4[ns]
        "#;
        let input_path: &str = "test_input.txt";
        let output_path: &str = "test_output.txt";
        let worker: &str = "worker1";

        create_temp_file(input, input_path).unwrap();

        let result: Result<(), io::Error> = ProcessFile::run(input_path, output_path, worker);

        assert!(result.is_ok());
        let output: String = read_temp_file(output_path).unwrap();
        assert!(output.contains("server/tests/something/thing3"));
        assert!(output.contains("server/tests/something/thing1"));
        assert!(output.contains("lib/tests/something/thing1"));

        // Clean up temporary files
        cleanup_temp_file(input_path);
        cleanup_temp_file(output_path);
    }

    #[test]
    fn test_process_file_no_matches() {
        let input: &str = r#"
        [worker2] tests/something/thing1
        "#;
        let input_path: &str = "test_input_no_match.txt";
        let output_path: &str = "test_output_no_match.txt";
        let worker: &str = "worker1";

        create_temp_file(input, input_path).unwrap();

        let result: Result<(), io::Error> = ProcessFile::run(input_path, output_path, worker);

        assert!(result.is_ok());
        let output: String = read_temp_file(output_path).unwrap();
        assert!(output.is_empty());

        // Clean up temporary files
        cleanup_temp_file(input_path);
        cleanup_temp_file(output_path);
    }
}
