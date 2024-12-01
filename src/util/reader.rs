use std::fs;

#[derive(Debug, PartialEq)]
pub struct Reader {
    pub filename: String,
    pub contents: Option<Vec<String>>,
}

impl Reader {
    pub fn new(filename: &'static str) -> Reader {
        Reader {
            filename: filename.to_string(),
            contents: None,
        }
    }

    pub fn read(&self) -> String {
        fs::read_to_string(self.filename.clone()).expect("Something went wrong reading the file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILENAME: &str = "test/fixtures/manylines";

    #[test]
    fn can_make_reader() {
        let reader = Reader::new(FILENAME);
        assert_eq!(reader.filename, FILENAME);
    }

    #[test]
    fn constructor_and_new_method_are_the_same() {
        let reader1 = Reader::new(FILENAME);
        let reader2 = Reader {
            filename: FILENAME.to_string(),
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
        let reader = Reader::new(FILENAME);
        let contents = reader.read();
        assert_eq!(contents, "Line 1\nLine 2\nLine 3\nLine 4");
    }
}
