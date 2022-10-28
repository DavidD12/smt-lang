use crate::parser::Position;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use line_col::LineColLookup;

pub enum Error {
    File {
        filename: String,
        message: String,
    },
    Parse {
        message: String,
        position: Option<Position>,
        expected: Vec<String>,
    },
    Duplicate {
        name: String,
        first: Option<Position>,
        second: Option<Position>,
    },
    Resolve {
        name: String,
        position: Option<Position>,
    },
}

impl Error {
    pub fn new_parse(
        file: &str,
        lookup: &LineColLookup,
        error: ParseError<usize, Token, &str>,
    ) -> Self {
        match error {
            ParseError::InvalidToken { location } => Self::Parse {
                message: "invalid token".into(),
                position: Some(Position::new(file, lookup, location)),
                expected: Vec::new(),
            },
            ParseError::UnrecognizedEOF { location, expected } => Self::Parse {
                message: "unreconized EOF".into(),
                position: Some(Position::new(file, lookup, location)),
                expected,
            },
            ParseError::UnrecognizedToken { token, expected } => Self::Parse {
                message: format!("unreconized token '{}'", token.1),
                position: Some(Position::new(file, lookup, token.0)),
                expected,
            },
            ParseError::ExtraToken { token } => Self::Parse {
                message: format!("extra token '{}'", token.1),
                position: Some(Position::new(file, lookup, token.0)),
                expected: Vec::new(),
            },
            ParseError::User { error } => Self::Parse {
                message: format!("parse error '{}'", error),
                position: None,
                expected: Vec::new(),
            },
        }
    }
}

impl crate::problem::ToLang for Error {
    fn to_lang(&self, model: &crate::problem::Problem) -> String {
        match self {
            Error::File { filename, message } => {
                format!("cannot read file {} {}", filename, message)
            }
            Error::Parse {
                message,
                position,
                expected,
            } => match position {
                Some(position) => format!(
                    "parse error '{}' at {}, expecting: {:?}",
                    message, position, expected
                ),
                None => format!("parse error '{}', expecting: {:?}", message, expected),
            },
            Error::Duplicate {
                name,
                first,
                second,
            } => match (first, second) {
                (None, None) => format!("duplicate '{}'", name),
                (None, Some(p)) => format!("duplicate '{}' at {}", name, p),
                (Some(p), None) => format!("duplicate '{}' at {}", name, p),
                (Some(p1), Some(p2)) => format!("duplicate '{}' at {} and {}", name, p1, p2),
            },
            Error::Resolve { name, position } => {
                if let Some(position) = position {
                    format!("unresolved {} at {}", name, position)
                } else {
                    format!("unresolved {}", name)
                }
            }
        }
    }
}
