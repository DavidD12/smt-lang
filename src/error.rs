use crate::parser::Position;
use crate::problem::*;
use d_stuff::Message;
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
        token: Option<String>,
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
    Interval {
        name: String,
        position: Option<Position>,
    },
    Type {
        expr: Expr,
        typ: Type,
        expected: Vec<Type>,
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
                message: "Invalid Token".into(),
                token: None,
                position: Some(Position::new(file, lookup, location)),
                expected: Vec::new(),
            },
            ParseError::UnrecognizedEOF { location, expected } => Self::Parse {
                message: "Unreconized EOF".into(),
                token: None,
                position: Some(Position::new(file, lookup, location)),
                expected,
            },
            ParseError::UnrecognizedToken { token, expected } => Self::Parse {
                message: "Unreconized Token".into(),
                token: Some(token.1.to_string()),
                position: Some(Position::new(file, lookup, token.0)),
                expected,
            },
            ParseError::ExtraToken { token } => Self::Parse {
                message: "Extra Token".into(),
                token: Some(token.1.to_string()),
                position: Some(Position::new(file, lookup, token.0)),
                expected: Vec::new(),
            },
            ParseError::User { error } => Self::Parse {
                message: "Parse Error".into(),
                token: Some(error.to_string()),
                position: None,
                expected: Vec::new(),
            },
        }
    }
}

fn expected_tokens(expected: &Vec<String>) -> String {
    let mut s = "".to_string();
    match expected.first() {
        None => return s,
        Some(t) => {
            s.push_str(&t);
            for t in expected[1..].iter() {
                s.push_str(&format!(" or {}", t));
            }
        }
    }
    s
}

fn expected_types(problem: &Problem, expected: &Vec<Type>) -> String {
    let mut s = "".to_string();
    match expected.first() {
        None => return s,
        Some(t) => {
            s.push_str(&t.to_lang(problem));
            for t in expected[1..].iter() {
                s.push_str(&format!(" or {}", t.to_lang(problem)));
            }
        }
    }
    s
}

//------------------------- ToLang -------------------------

impl ToLang for Error {
    fn to_lang(&self, problem: &crate::problem::Problem) -> String {
        match self {
            Error::File { filename, message } => {
                format!("cannot read file {} {}", filename, message)
            }
            Error::Parse {
                message,
                token,
                position,
                expected,
            } => match position {
                Some(position) => format!(
                    "parse error '{}' at {}, expecting: {:?}",
                    message, position, expected
                ),
                None => format!(
                    "parse error '{}', expecting: {}",
                    message,
                    expected_tokens(expected)
                ),
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
            Error::Interval { name, position } => {
                if let Some(position) = position {
                    format!("malformed interval {} at {}", name, position)
                } else {
                    format!("malformed interval {}", name)
                }
            }
            Error::Type {
                expr,
                typ,
                expected,
            } => {
                let mut s = if !expected.is_empty() {
                    format!(
                        "type error: '{}' type is '{}' but expecting '{}'",
                        expr.to_lang(problem),
                        typ.to_lang(problem),
                        expected_types(problem, expected)
                    )
                } else {
                    format!(
                        "type error: '{}' type is '{}'",
                        expr.to_lang(problem),
                        typ.to_lang(problem)
                    )
                };
                if let Some(p) = expr.position() {
                    s.push_str(&format!(" at {}", p));
                }
                s
            }
        }
    }
}

//------------------------- To Entry -------------------------

pub fn expected_to_message(expected: &Vec<String>) -> d_stuff::Message {
    let title = d_stuff::Text::new(
        "Expexted",
        termion::style::Reset.to_string(),
        termion::color::White.fg_str(),
    );

    let mut s = "".to_string();
    if let Some((first, others)) = expected.split_first() {
        s.push_str(first);
        for x in others {
            s.push_str(&format!(" {}", x));
        }
    }
    let message = d_stuff::Text::new(
        s,
        termion::style::Reset.to_string(),
        termion::color::LightBlue.fg_str(),
    );
    d_stuff::Message::new(Some(title), message)
}

impl ToEntry for Error {
    fn to_entry(&self, problem: &Problem) -> d_stuff::Entry {
        match self {
            Error::File { filename, message } => d_stuff::Entry::new(
                d_stuff::Status::Failure,
                d_stuff::Text::new(
                    "File",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "ERROR",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![
                    d_stuff::Message::new(
                        Some(d_stuff::Text::new(
                            "Cannot Read File",
                            termion::style::Reset.to_string(),
                            termion::color::Red.fg_str(),
                        )),
                        d_stuff::Text::new(
                            filename,
                            termion::style::Reset.to_string(),
                            termion::color::Cyan.fg_str(),
                        ),
                    ),
                    d_stuff::Message::new(
                        Some(d_stuff::Text::new(
                            "Message",
                            termion::style::Reset.to_string(),
                            termion::color::White.fg_str(),
                        )),
                        d_stuff::Text::new(
                            message,
                            termion::style::Reset.to_string(),
                            termion::color::LightBlue.fg_str(),
                        ),
                    ),
                ],
            ),
            Error::Parse {
                message,
                token,
                position,
                expected,
            } => {
                let mut messages = vec![];

                let title = d_stuff::Text::new(
                    message,
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                );
                if let Some(token) = token {
                    messages.push(d_stuff::Message::new(
                        Some(title),
                        d_stuff::Text::new(
                            format!("'{}'", token),
                            termion::style::Reset.to_string(),
                            termion::color::LightBlue.fg_str(),
                        ),
                    ))
                } else {
                    messages.push(d_stuff::Message::new(None, title));
                }
                if let Some(position) = position {
                    messages.push(position.to_message());
                }
                if !expected.is_empty() {
                    messages.push(expected_to_message(expected));
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Parse",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }

            Error::Duplicate {
                name,
                first,
                second,
            } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(d_stuff::Text::new(
                        "Defined Twice",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = first {
                    messages.push(position.to_message());
                }
                if let Some(position) = second {
                    messages.push(position.to_message());
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Unicity",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Resolve { name, position } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(d_stuff::Text::new(
                        "Undefined Identifier",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = position {
                    messages.push(position.to_message());
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Resolve",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Interval { name, position } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(d_stuff::Text::new(
                        "Malformed Interval",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = position {
                    messages.push(position.to_message());
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Interval",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Type {
                expr,
                typ,
                expected,
            } => {
                let mut messages = vec![];

                messages.push(d_stuff::Message::new(
                    Some(d_stuff::Text::new(
                        "Type Error",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", expr.to_lang(problem)),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));

                if let Some(position) = expr.position() {
                    messages.push(position.to_message());
                }

                messages.push(d_stuff::Message::new(
                    Some(d_stuff::Text::new(
                        "Type",
                        termion::style::Reset.to_string(),
                        termion::color::White.fg_str(),
                    )),
                    d_stuff::Text::new(
                        typ.to_lang(problem),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));

                if !expected.is_empty() {
                    messages.push(expected_to_message(
                        &expected.iter().map(|t| t.to_lang(problem)).collect(),
                    ));
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Type",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
        }
    }
}
