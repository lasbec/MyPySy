use std::collections::HashMap;

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


pub fn lex(input: &String) -> Result<Vec<MetaToken>, String>    {
    let mut result:Vec<MetaToken> = Vec::new();

    let mut words = HashMap::from([
        ("true".to_string(),  Token::True("true".to_string())),
        ("false".to_string(), Token::False("false".to_string())),
        ("if".to_string(),    Token::If("if".to_string())),
        ("else".to_string(),  Token::Else("else".to_string())),
        ("while".to_string(), Token::While("while".to_string())),
    ]);

    let mut it = input.chars().peekable();

    let mut _lineno = 1;

    while let Some(&c) = it.peek()  {
        match c {
            ' ' | '\t' => {
                it.next();
            },
            '\n'  =>  {
                _lineno += 1;
                it.next();
            },
            '&' =>  {
                it.next();
                let ch = it.peek();
                if let Some('&') = ch   {
                    result.push(MetaToken { 
                        token: Token::And("&&".to_string()),
                        line_no: _lineno,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Id("&".to_string()),
                        line_no: _lineno,
                    });
                };
            },
            '|' =>  {
                it.next();
                let ch = it.peek();
                if let Some('|') = ch   {
                    result.push(MetaToken {
                        token: Token::Or("||".to_string()),
                        line_no: _lineno,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Id("|".to_string()),
                        line_no: _lineno
                    });
                };
            },
            '=' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken {
                        token: Token::Eql("==".to_string()),
                        line_no: _lineno,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken { 
                        token:Token::Id("=".to_string()),
                        line_no: _lineno,
                    });
                };
            },
            '!' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken { 
                        token: Token::Ne("!=".to_string()),
                        line_no: _lineno
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Id("!".to_string()),
                        line_no: _lineno,
                    });
                };
            },
            '<' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken {
                        token: Token::Le("<=".to_string()),
                        line_no: _lineno,

                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Lt("<".to_string()),
                        line_no: _lineno,
                    });
                };
            },
            '>' =>  {
                it.next();
                let ch = it.peek();
                if let Some('=') = ch   {
                    result.push(MetaToken {
                        token: Token::Ge(">=".to_string()),
                        line_no: _lineno,
                    });
                    it.next();
                } else  {
                    result.push(MetaToken {
                        token: Token::Gt(">".to_string()),
                        line_no: _lineno
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
                    line_no: _lineno,
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
                        line_no: _lineno,
                    }),
                    None => {
                        result.push(MetaToken {
                            token:Token::Id(s.clone()),
                            line_no: _lineno
                        });
                        words.insert(s.clone(), Token::Id(s.clone()));
                    },
                }
            },
            _ => {
                result.push(MetaToken { 
                    token: Token::Id(c.to_string()),
                    line_no: _lineno,
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
    let input = String::from("1 _ while != && =ok 3.4 1.0=_ true false if else true1");
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
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Id("=".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Id("_".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::True("true".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::False("false".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::If("if".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Else("else".to_string()),
                    line_no: 1,
                },
                MetaToken {
                    token: Token::Id("true1".to_string()),
                    line_no: 1,
                },
            ];
            assert_eq!(expected, r);
        },
        Err(_) => println!("Error getting the return value."),
    }
}
