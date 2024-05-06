#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    const PROGRAM: &str = "target/debug/quickreplace";

    #[test]
    fn should_error_when_arguments_is_missing() {
        let output = Command::new(PROGRAM)
            .output()
            .expect("Failed to run process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), "quickreplace - change occurrences of one string into another\nUsage: quickreplace <target> <replacement> <filename> <output>\nError: wrong number of arguments: expected 4, got 0\n");
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn should_error_when_only_one_argument() {
        let output = Command::new(PROGRAM)
            .arg("target")
            .output()
            .expect("Failed to run process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), "quickreplace - change occurrences of one string into another\nUsage: quickreplace <target> <replacement> <filename> <output>\nError: wrong number of arguments: expected 4, got 1\n");
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn should_error_when_only_two_argument() {
        let output = Command::new(PROGRAM)
            .arg("target")
            .arg("replacement")
            .output()
            .expect("Failed to run process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), "quickreplace - change occurrences of one string into another\nUsage: quickreplace <target> <replacement> <filename> <output>\nError: wrong number of arguments: expected 4, got 2\n");
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn should_error_when_only_three_argument() {
        let output = Command::new(PROGRAM)
            .arg("target")
            .arg("replacement")
            .arg("filename")
            .output()
            .expect("Failed to run process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), "quickreplace - change occurrences of one string into another\nUsage: quickreplace <target> <replacement> <filename> <output>\nError: wrong number of arguments: expected 4, got 3\n");
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn should_error_when_file_not_found() {
        let output = Command::new(PROGRAM)
            .arg("target")
            .arg("replacement")
            .arg("filename")
            .arg("output")
            .output()
            .expect("Failed to run process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), "Error: failed to read from file 'filename': Os { code: 2, kind: NotFound, message: \"No such file or directory\" }\n");
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn should_replace_text() {
        let filename = "sample.txt";
        let output_filename = "output.txt";

        fs::write(filename, "Hello, world!").expect("Failed to write file");

        let output = Command::new(PROGRAM)
            .arg("world")
            .arg("Rust")
            .arg(filename)
            .arg(output_filename)
            .output()
            .expect("Failed to run process");

        assert_eq!(output.status.code(), Some(0));

        let content = fs::read_to_string(output_filename).expect("Failed to read file");
        assert_eq!(content, "Hello, Rust!");

        fs::remove_file(filename).expect("Failed to remove file");
        fs::remove_file(output_filename).expect("Failed to remove file");
    }
}
