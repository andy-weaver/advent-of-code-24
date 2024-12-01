use std::fs;

const FILENAME: &str = "~/rust/advent-of-code-24/input/01-historian-histeria.input";

#[derive(Debug, PartialEq)]
pub struct Reader {
    pub filename: &'static str,
    pub contents: Option<Vec<String>>,
}

impl Reader {
    pub fn new(filename: &'static str) -> Reader {
        Reader {
            filename,
            contents: None,
        }
    }

    pub fn read(&self) -> String {
        fs::read_to_string(self.filename).expect("Something went wrong reading the file")
    }

    pub fn split_lines(&self) -> Vec<String> {
        let contents = self.read();
        contents.lines().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_reader() {
        let reader = Reader::new(FILENAME);
        assert_eq!(reader.filename, FILENAME);
    }

    #[test]
    fn constructor_and_new_method_are_the_same() {
        let reader1 = Reader::new(FILENAME);
        let reader2 = Reader {
            filename: FILENAME,
            contents: None,
        };

        assert_eq!(reader1, reader2);
    }

    #[test]
    fn reader_can_read_a_simple_file() {
        let reader = Reader::new("test/fixtures/simplefile");
        let contents = reader.read();
        assert_eq!(contents, "Hello, reader.");
    }

    #[test]
    #[should_panic(expected = "Something went wrong reading the file")]
    fn reader_tells_me_when_something_goes_wrong_with_reading_the_file() {
        let reader = Reader::new("test/fixtures/nonexistentfile");
        reader.read();
    }

    #[test]
    fn reader_can_read_a_file_with_multiple_lines() {
        let reader = Reader::new("test/fixtures/manylines");
        let contents = reader.read();
        assert_eq!(contents, "Line 1\nLine 2\nLine 3\nLine 4");
    }

    #[test]
    fn reader_can_split_by_line() {
        let reader = Reader::new("test/fixtures/manylines");
        let lines: Vec<String> = reader.split_lines();
        assert_eq!(
            lines,
            vec![
                "Line 1".to_string(),
                "Line 2".to_string(),
                "Line 3".to_string(),
                "Line 4".to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn reader_can_split_input_by_whitespace() {
        // let mut reader = Reader::new("test/fixtures/manylines");
        // let delimiters = vec!["\n", " "];
        // let lines: Vec<&str> = reader.split_at(delimiters);
        // assert_eq!(
        //     lines,
        //     vec!["Line", "1", "Line", "2", "Line", "3", "Line", "4"]
        // );
    }
}
