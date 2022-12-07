const INDENT: char = '\t';
const INDENT_LEN: usize = INDENT.len_utf8();

#[derive(Debug)]
struct Line(usize, usize); // start, end

impl Line {
    fn from_str(src: &str) -> Vec<Self> {
        let mut lines = Vec::new();
        let mut last_index = 0;
        let mut current_index = 0;
        for c in src.chars() {
            if c == '\n' {
                lines.push(Self(last_index, current_index));
                last_index = current_index + 1;
            }
            current_index += 1;
        }
        lines.push(Self(last_index, current_index));
        lines
    }
}

pub struct ColumnReader<'a> {
    src: &'a str,
    lines: Vec<Line>,
    pub(crate) line: usize,
    pub(crate) pos: usize,
}

impl<'a> ColumnReader<'a> {
    /// Creates a new [`ColumnReader`] from a [`String`].
    pub fn new(src: &'a str) -> Self {
        Self {
            lines: Line::from_str(src),
            src,
            line: 1,
            pos: 0,
        }
    }

    /// Returns `true` if the reader is not EOF the next line is indented by `indent` tabs.
    pub fn next_line(&mut self, indent: usize) -> bool {
        let Some(line) = self.line_at(self.line + 1) else {
            return false;
        };
        if line.len() < indent || !next_n_is_indent(line, indent) {
            return false;
        }
        self.line += 1;
        self.reset_pos(indent * INDENT_LEN);
        true
    }

    /// Returns `true` if the next column is equal to `expect`.
    pub fn next_col_expect(&mut self, expect: &str) -> bool {
        self.peek_next_col() == Some(expect)
    }

    /// Returns the next column and advances the reader.
    /// Returns [`None`] if EOL or EOF is reached.
    pub fn next_col(&mut self) -> Option<&'a str> {
        self.peek_next_col().map(|col| {
            self.advance(col.len() + 1);
            col
        })
    }

    /// Returns the next column without advancing the reader.
    /// Returns [`None`] if EOL or EOF is reached.
    pub fn peek_next_col(&self) -> Option<&'a str> {
        let pos = self.lines.get(self.line - 1).expect("line out of bounds");
        let line = &self.src[pos.0..pos.1];
        if self.pos >= line.len() {
            return None; // eol reached
        }
        line[self.pos..].splitn(2, ' ').next()
    }

    /// Returns the next column without advancing the reader.
    pub fn peek_next_cols(&self) -> &'a str {
        if self.eof() {
            return "";
        }
        &self.peek_line()[self.pos..]
    }

    /// Returns the current line without advancing the reader.
    pub fn peek_line(&self) -> &'a str {
        self.line_at(self.line).expect("line out of bounds")
    }

    /// Returns `true` if EOF or EOL is reached.
    pub fn eof(&self) -> bool {
        self.line == self.lines.len() || self.pos >= self.peek_line().len()
    }

    // expects line to be 1-indexed
    fn line_at(&self, line: usize) -> Option<&'a str> {
        self.lines
            .get(line - 1)
            .map(|line| &self.src[line.0..line.1])
    }

    fn reset_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn advance(&mut self, i: usize) {
        self.pos += i;
    }
}

fn next_n_is_indent(line: &str, offset: usize) -> bool {
    offset == 0 || line[..offset].chars().all(|c| c == INDENT)
}

#[cfg(test)]
mod test {
    use super::ColumnReader;

    #[test]
    fn test_column_reader() {
        let mut reader = ColumnReader::new("a b c\nd e f\ng h i".into());
        assert_eq!(reader.peek_line(), "a b c");
        assert_eq!(reader.peek_next_col(), Some("a"));
        assert_eq!(reader.next_col(), Some("a"));
        assert_eq!(reader.next_col(), Some("b"));
        assert_eq!(reader.next_col(), Some("c"));
        assert_eq!(reader.next_col(), None);
        assert!(reader.next_line(0));
        assert_eq!(reader.peek_line(), "d e f");
        assert!(reader.next_col_expect("d"));
        assert!(reader.next_col_expect("e"));
        assert!(reader.next_col_expect("f"));
        assert_eq!(reader.next_col(), Some("d"));
        assert_eq!(reader.next_col(), Some("e"));
        assert_eq!(reader.next_col(), Some("f"));
        assert_eq!(reader.next_col_expect("something"), false); // no more columns
        assert!(reader.next_line(0));
        assert_eq!(reader.peek_line(), "g h i");
        assert!(reader.next_col_expect("g"));
        assert!(reader.next_col_expect("h"));
        assert!(reader.next_col_expect("i"));
        assert_eq!(reader.next_line(0), false);
    }

    #[test]
    fn test_indent() {
        let mut reader = ColumnReader::new("CLASS\n	METHOD".into());
        assert_eq!(reader.peek_line(), "CLASS");
        assert_eq!(reader.next_col(), Some("CLASS"));
        assert_eq!(reader.next_col(), None);
        assert!(reader.next_line(1));
        assert_eq!(reader.peek_next_cols(), "METHOD");
        assert_eq!(reader.next_col(), Some("METHOD"));
        assert_eq!(reader.next_col(), None);
        assert_eq!(reader.next_line(0), false);
    }
}
