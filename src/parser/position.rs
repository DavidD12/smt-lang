use line_col::LineColLookup;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new<S: Into<String>>(file: S, lookup: &LineColLookup, offset: usize) -> Self {
        let file = file.into();
        let (line, column) = lookup.get(offset);
        Self { file, line, column }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}:{}", self.file, self.line, self.column)
    }
}
