use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::convert::From;

#[derive(Debug, PartialEq, Eq)]
pub enum PathElementType {
    ArrayIndex,
    HashMapKey,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Key(String),
    Index(i64),
}

impl<'a> From<&'a str> for Token {

    fn from(s: &'a str) -> Token {
        String::from(s).into()
    }

}

impl From<String> for Token {

    fn from(s: String) -> Token {
        use std::str::FromStr;

        if s.chars().next() == Some('[') && s.chars().last() == Some(']') {
            let u = FromStr::from_str(&s[..]);
            if u.is_err() {
                Token::Index(-1)
            } else {
                Token::Index(u.unwrap())
            }
        } else {
            Token::Key(s)
        }
    }
}

impl Into<PathElement> for Token {

    fn into(self) -> PathElement {
        match self {
            Token::Key(_) => {
                PathElement {
                    pe_name: self,
                    pe_type: PathElementType::HashMapKey,
                }
            },

            Token::Index(_) => {
                PathElement {
                    pe_name: self,
                    pe_type: PathElementType::ArrayIndex,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PathElement {
    pe_name: Token,
    pe_type: PathElementType,
}

#[derive(Debug)]
pub struct ParserError {
    cause: Option<Box<Error>>,
}

impl ParserError {

    pub fn new(cause: Option<Box<Error>>) -> ParserError {
        ParserError {
            cause: cause,
        }
    }

}

impl Display for ParserError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[ParserError]", ));
        Ok(())
    }

}

impl Error for ParserError {

    fn description(&self) -> &str {
        "ParserError"
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

pub fn compile(source: &str) -> Result<Vec<PathElement>, ParserError> {
    tokenize(String::from(source))
        .map(|tokenize_res| {
            tokenize_res.into_iter().map(|t| t.into()).collect()
        })
        .map_err(|e| ParserError::new(Some(Box::new(e))))
}

fn tokenize(source: String) -> Result<Vec<Token>, ParserError> {
    Ok(source
        .split(".")
        .map(|sub| {
            let t : Token = String::from(sub).into();
            match t {
                Token::Index(i) => {
                    if i < 0 {
                        return Err(ParserError::new(None));
                    } else {
                        Ok(Token::Index(i))
                    }
                },
                other => Ok(other),
            }
        })
        .filter_map(Result::ok)
        .collect())
}

#[cfg(test)]
mod test {
    use super::{PathElementType, PathElement, compile};

    #[test]
    fn test_simple() {
        let path = "";
        let exp  = vec![
            PathElement {
                pe_name: "".into(),
                pe_type: PathElementType::HashMapKey,
            }
        ];

        assert_eq!(compile(path).unwrap(), exp);
    }


    #[test]
    fn test_simple_path() {
        let path = "a";
        let exp  = vec![
            PathElement {
                pe_name: "a".into(),
                pe_type: PathElementType::HashMapKey,
            }
        ];

        assert_eq!(exp, compile(path).unwrap());
    }

    #[test]
    fn test_simple_path_with_sub() {
        let path = "a.b";
        let exp  = vec![
            PathElement {
                pe_name: "a".into(),
                pe_type: PathElementType::HashMapKey,
            },
            PathElement {
                pe_name: "b".into(),
                pe_type: PathElementType::HashMapKey,
            },
        ];

        assert_eq!(exp, compile(path).unwrap());
    }
}

