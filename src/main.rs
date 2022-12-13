use std::collections::{HashMap, binary_heap::Iter};

fn main(){
    print!("Hello Lex");
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token    {
    Num(f64),
    Id(String),
    True(String),
    False(String),
    If(String),
    Else(String),
    While(String),
    And(String),
    Or(String),
    Eql(String),
    Ne(String),
    Le(String),
    Ge(String),
    Lt(String),
    Gt(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaToken {
    token: Token,
    line_no: i32,
}

pub trait PeekableIterator : std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

fn lex_special_sign(it: &mut impl PeekableIterator<Item = char>, result: &mut Vec<MetaToken>, line_no:i32) {
    it.next();
    let ch = it.peek();
    if let Some('&') = ch   {
        result.push(MetaToken { 
            token: Token::And("&&".to_string()),
            line_no,
        });
        it.next();
    } else  {
        result.push(MetaToken {
            token: Token::Id("&".to_string()),
            line_no,
        });
    };
}

fn lex_special_sign_lt(it: &mut impl PeekableIterator<Item = char>, result: &mut Vec<MetaToken>, line_no:i32) {
    it.next();
    let ch = it.peek();
    match ch {
        Some('=') => {
            result.push(MetaToken {
                token: Token::Le("<=".to_string()),
                line_no,
    
            });
            it.next();
        },
        _ => {
            result.push(MetaToken {
                token: Token::Lt("<".to_string()),
                line_no,
            });
        }
    }
}


pub fn lex(input: &String) -> Result<Vec<MetaToken>, String>    {
    let mut result: Vec<MetaToken> = Vec::new();

    let mut words = HashMap::from([
        ("true".to_string(),  Token::True("true".to_string())),
        ("false".to_string(), Token::False("false".to_string())),
        ("if".to_string(),    Token::If("if".to_string())),
        ("else".to_string(),  Token::Else("else".to_string())),
        ("while".to_string(), Token::While("while".to_string())),
    ]);

    let mut it = input.chars().peekable();

    let mut line_no = 1;

    while let Some(&c) = it.peek()  {
        match c {
            ' ' | '\t' => {
                it.next();
            },
            '\n'  =>  {
                line_no += 1;
                it.next();
            },
            '&' =>  {
                lex_special_sign(&mut it, &mut result, line_no);
            },
            '|' =>  {
                it.next();
                let ch = it.peek();
                if let Some('|') = ch   {
                    result.push(MetaToken {
                        token: Token::Or("||".to_string()),
                        line_no,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Id("|".to_string()),
                        line_no
                    });
                };
            },
            '=' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken {
                        token: Token::Eql("==".to_string()),
                        line_no,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken { 
                        token:Token::Id("=".to_string()),
                        line_no,
                    });
                };
            },
            '!' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken { 
                        token: Token::Ne("!=".to_string()),
                        line_no
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Id("!".to_string()),
                        line_no,
                    });
                };
            },
            '<' =>  lex_special_sign_lt(&mut it, &mut result, line_no),
            '>' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken {
                        token: Token::Ge(">=".to_string()),
                        line_no,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Gt(">".to_string()),
                        line_no
                    });
                }
            }
            '0'..='9' =>    {
                let mut n = c.to_string().parse::<f64>().expect("Character not a digit.");

                it.next();
                let mut digitch = it.peek();

                while let Some(&i) = digitch {
                    if !i.is_digit(10)   {
                        if i == '.'    {
                            let mut d = 10.0;
                            it.next();
                            digitch = it.peek();

                            while let Some(&j) = digitch    {
                                if !j.is_digit(10) {
                                    digitch = None;
                                } else  {
                                    let f = j.to_string().parse::<f64>().expect("Character not a digit.");
                                    n = n + f / d;
                                    d = d * 10.0;
                                    it.next();
                                    digitch = it.peek();
                                }
                            }
                        } else  {
                            digitch = None;
                        }
                    } else  {
                        let digit = i.to_string().parse::<f64>().expect("Character not a digit.");
                        n = n*10.0 + digit;
                        it.next();
                        digitch = it.peek();
                    }
                }
                result.push(MetaToken {
                    token: Token::Num(n),
                    line_no,
                });
            },
            'A'..='Z' | 'a'..='z' => {
                let mut s = String::new();
                s.push(c);

                it.next();
                let mut ch = it.peek();
                while let Some(&i) = ch {
                    if !i.is_digit(10) && !i.is_alphabetic()  {
                        ch = None;
                    } else  {
                        s.push(i);
                        it.next();
                        ch = it.peek();
                    }
                }
                println!("{}", s);
                match words.get(&s)  {
                    Some(t) => result.push(MetaToken {
                        token: Token::clone(t),
                        line_no,
                    }),
                    None => {
                        result.push(MetaToken {
                            token:Token::Id(s.clone()),
                            line_no
                        });
                        words.insert(s.clone(), Token::Id(s.clone()));
                    },
                }
            },
            _ => {
                result.push(MetaToken { 
                    token: Token::Id(c.to_string()),
                    line_no,
                });
                it.next();
            },
        }
    }

    return Ok(result);
}


#[test]
fn correct_amount_of_tokens()   {
    let input = String::from("1 _ != && =ok 3.4 1.0=_");
    let result = lex(&input);
    match result    {
        Ok(r) => assert_eq!(10, r.len()),
        Err(_) => println!("Error getting the return value."),
    }
}

#[test]
fn correct_token_types()    {
    let input = String::from("1 _ while != && =ok 3.4 \n1.0=_ true false if else true1");
    let result = lex(&input);
    match result    {
        Ok(r) =>    {
            let expected = vec![
                MetaToken {
                    token: Token::Num(1.0),
                    line_no: 1
                },
                MetaToken {
                    token: Token::Id("_".to_string()),
                    line_no: 1
                },
                MetaToken {
                    token: Token::While("while".to_string()),
                    line_no: 1
                },
                MetaToken {
                    token: Token::Ne("!=".to_string()),
                    line_no: 1
                },
                MetaToken {
                    token: Token::And("&&".to_string()),
                    line_no: 1
                },
                MetaToken {
                    token: Token::Id("=".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Id("ok".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Num(3.4),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Num(1.0),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::Id("=".to_string()),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::Id("_".to_string()),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::True("true".to_string()),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::False("false".to_string()),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::If("if".to_string()),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::Else("else".to_string()),
                    line_no: 2,
                },
                MetaToken {
                    token: Token::Id("true1".to_string()),
                    line_no: 2,
                },
            ];
            assert_eq!(expected, r);
        },
        Err(_) => println!("Error getting the return value."),
    }
}
