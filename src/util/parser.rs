pub trait Parsable {
    fn parse(&self, delimiter: &str) -> Vec<Vec<&str>>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    pub raw: String,
}

impl Parser {
    pub fn new(raw: &str) -> Parser {
        Parser {
            raw: raw.to_string(),
        }
    }

    pub fn split_lines(&self) -> Vec<String> {
        self.raw.lines().map(|s| s.to_string()).collect()
    }

    pub fn split_at(&self, delimiters: Vec<&str>) -> Vec<String> {
        let mut split = vec![self.raw.clone()];
        for delimiter in delimiters {
            split = split
                .iter()
                .flat_map(|s| s.split(delimiter))
                .map(|s| s.to_string())
                .collect();
        }
        split
    }
}

impl Parsable for Parser {
    fn parse(&self, delimiter: &str) -> Vec<Vec<&str>> {
        self.raw
            .lines()
            .map(|line| {
                if line.is_empty() {
                    return vec![];
                }
                line.split(delimiter).collect()
            })
            .filter(|line| !line.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::util::Reader;

    fn get_file_contents(filename: &'static str) -> String {
        let reader = Reader::new(filename);
        reader.read()
    }

    fn get_expected_parse_output() -> Vec<Vec<&'static str>> {
        vec![
            vec!["item1a", "item1b"],
            vec!["item2a", "item2b"],
            vec!["item3a", "item3b"],
        ]
    }

    fn can_parse_by_delimiter(raw_test_case: &str, delimiter: &str) {
        let parser = Parser::new(raw_test_case);
        let actual_parsed = parser.parse(delimiter);
        let expected = get_expected_parse_output();
        assert_eq!(actual_parsed, expected);
    }

    #[test]
    fn can_create_parser_with_new_method() {
        let parser = Parser::new("Hello, parser.");
        assert_eq!(parser.raw, "Hello, parser.");
    }

    #[test]
    fn constructor_is_identical_to_new_method() {
        let parser1 = Parser::new("Hello, parser.");
        let parser2 = Parser {
            raw: "Hello, parser.".to_string(),
        };
        assert_eq!(parser1, parser2);
    }

    #[test]
    fn parser_can_split_by_line() {
        let file = get_file_contents("test/fixtures/manylines");
        let parser = Parser::new(&file);
        let lines: Vec<String> = parser.split_lines();
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
    fn can_split_by_vec_of_delimiters() {
        let parser = Parser::new("item1,item2 item3");
        let delimiters = vec![" ", ","];
        let split = parser.split_at(delimiters);
        assert_eq!(split, vec!["item1", "item2", "item3"]);
    }

    #[test]
    fn can_parse_by_comma() {
        can_parse_by_delimiter("item1a,item1b\nitem2a,item2b\nitem3a,item3b", ",");
    }

    #[test]
    fn can_parse_by_space() {
        can_parse_by_delimiter("item1a item1b\nitem2a item2b\nitem3a item3b", " ");
    }

    #[test]
    fn can_parse_by_tab() {
        can_parse_by_delimiter("item1a\titem1b\nitem2a\titem2b\nitem3a\titem3b", "\t");
    }

    #[test]
    fn parser_will_ignore_empty_lines_if_they_exist() {
        can_parse_by_delimiter("item1a,item1b\n\nitem2a,item2b\nitem3a,item3b", ",");
    }
}
