use std::collections::{HashMap, binary_heap::Iter};

fn main(){
    print!("Hello Lex");
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token    {
    Num(f64),
    Id,
    True,
    False,
    If,
    Else,
    While,
    And,
    Or,
    Eql,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaToken {
    content: String,
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

fn lex_keyword(prefix_tree: &PrefixTree, it: &mut impl PeekableIterator<Item = char>, content: &mut String) -> Option<Token> {
    let c_opt = it.peek();
    match c_opt {
        Some(c) => {
            let childTree = prefix_tree.get_child(c);
            match childTree {
                None => {
                    match prefix_tree {
                        PrefixTree::Root(_) => None,
                        PrefixTree::Leaf(token) => {
                            match it.next() {
                                None => panic!("ABC 0"),
                                Some(_) => {
                                    Some(token.clone())
                                }
                            }
                            
                        },
                        PrefixTree::Node(token, _) => {
                            match it.next() {
                                None => panic!("ABC 1"),
                                Some(_) => {
                                    Some(token.clone())
                                }
                            }
                        }
                    }
                },
                Some(child) => {
                    match it.next() {
                        None => panic!("ABC 2"),
                        Some(c) => {
                            content.push(c);
                            lex_keyword(child, it, content)
                        }
                    }
                    
                }
            }
        },
        None => None
    }
}

fn lex_special_sign(first_char:&char,first_token:Token, second_char:&char, second_token:Token, it: &mut impl PeekableIterator<Item = char>, result: &mut Vec<MetaToken>, line_no:i32) {
    it.next();
    let ch = it.peek();
    let mut content = first_char.to_string();
    match ch {
        Some(n) => {
            if n == second_char{
                content.push(second_char.clone());
                result.push(MetaToken {
                    content,
                    token: second_token,
                    line_no,
        
                });
                it.next();
            } else {
                result.push(MetaToken {
                content,
                token: first_token,
                line_no,
            });
            }
        },
        None => {
            result.push(MetaToken {
                content,
                token: first_token,
                line_no,
            });
        }
    }
}

enum PrefixTree {
    Root(HashMap::<char, PrefixTree>),
    Leaf(Token),
    Node(Token, HashMap::<char, PrefixTree>)
}

impl PrefixTree {
    fn new() -> PrefixTree {
        return PrefixTree::Root(HashMap::new());
    }

    fn add_leaf(&mut self, c:char ,token:Token) -> &mut PrefixTree {
        let leaf = PrefixTree::Leaf(token.clone());
        match self {
            PrefixTree::Root(map) => {
                map.insert(c, leaf);
            },
            PrefixTree::Leaf(token) => {
                let mut map = HashMap::new();
                map.insert(c, leaf);
                *self = PrefixTree::Node(token.clone(), map);
            },
            PrefixTree::Node(token, map) => {
                map.insert(c, leaf);
            }
        }
        let result =  self.get_mut_child(&c);
        match result {
            Some(newChild) => newChild,
            None => panic!("ABc"),
        }

    }
    fn get_child(&self,c:&char) -> Option<&PrefixTree> {
        match self {
            PrefixTree::Root(map) => {
                map.get(c)
            },
            PrefixTree::Node(_, map) => {
                map.get(c)
            }
            PrefixTree::Leaf(_) => {
                None
            },
        }
    }

    fn get_mut_child(&mut self,c:&char) -> Option<&mut PrefixTree> {
        match self {
            PrefixTree::Root(map) => {
                map.get_mut(c)
            },
            PrefixTree::Node(_, map) => {
                map.get_mut(c)
            }
            PrefixTree::Leaf(_) => {
                None
            },
        }
    }
}



pub fn lex(input: &String) -> Result<Vec<MetaToken>, String> {
    let mut result: Vec<MetaToken> = Vec::new();
    let mut prefixMap = PrefixTree::new();
    prefixMap.add_leaf('&',Token::Id ).add_leaf('&',Token::And);
    prefixMap.add_leaf('|',Token::Id ).add_leaf('|',Token::Or);
    prefixMap.add_leaf('=',Token::Id ).add_leaf('=',Token::Eql);
    prefixMap.add_leaf('!',Token::Id).add_leaf('=',Token::Ne);
    prefixMap.add_leaf('<',Token::Lt).add_leaf('=',Token::Le);
    prefixMap.add_leaf('>',Token::Gt).add_leaf('=',Token::Ge);

    let mut words = HashMap::from([
        ("true".to_string(),  Token::True),
        ("false".to_string(), Token::False),
        ("if".to_string(),    Token::If),
        ("else".to_string(),  Token::Else),
        ("while".to_string(), Token::While),
    ]);

    let mut it = input.chars().peekable();

    let mut line_no = 1;

    while let Some(&c) = it.peek()  {
        /*let mut content = String::new();
        let token = lex_keyword(&prefixMap, &mut it, &mut content);
        match token {
            None => {},
            Some(t) => {
                result.push(MetaToken{
                    content,
                    token: t,
                    line_no
                });
            }
        }*/
        match c {
            ' ' | '\t' => {
                it.next();
            },
            '\n'  =>  {
                line_no += 1;
                it.next();
            },
            '&' => lex_special_sign(&'&',Token::Id ,&'&',Token::And,&mut it, &mut result, line_no),
            '|' => lex_special_sign(&'|',Token::Id ,&'|',Token::Or,&mut it, &mut result, line_no),
            '=' => lex_special_sign(&'=',Token::Id ,&'=',Token::Eql,&mut it, &mut result, line_no),
            '!' => lex_special_sign(&'!',Token::Id,&'=',Token::Ne,&mut it, &mut result, line_no),
            '<' => lex_special_sign(&'<',Token::Lt,&'=',Token::Le,&mut it, &mut result, line_no),
            '>' => lex_special_sign(&'>',Token::Gt,&'=',Token::Ge,&mut it, &mut result, line_no),
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
                    content: n.to_string(),
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
                        content: s.clone(),
                        token: Token::clone(t),
                        line_no,
                    }),
                    None => {
                        result.push(MetaToken {
                            content: s.clone(),
                            token:Token::Id,
                            line_no
                        });
                        words.insert(s.clone(), Token::Id);
                    },
                }
            },
            _ => {
                result.push(MetaToken { 
                    content: c.to_string(),
                    token: Token::Id,
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
fn map_token_types() {
    let input = String::from("!= ");
    let result = lex(&input);
    match result {
        Ok(r) => {
            let expected = vec![
                MetaToken {
                    content: "!=".to_string(),
                    token: Token::Ne,
                    line_no: 1
            }];
            assert_eq!(expected, r);
        },
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
                    content: "1".to_string(),
                    token: Token::Num(1.0),
                    line_no: 1
                },
                MetaToken {
                    content: "_".to_string(),
                    token: Token::Id,
                    line_no: 1
                },
                MetaToken {
                    content: "while".to_string(),
                    token: Token::While,
                    line_no: 1
                },
                MetaToken {
                    content: "!=".to_string(),
                    token: Token::Ne,
                    line_no: 1
                },
                MetaToken {
                    content: "&&".to_string(),
                    token: Token::And,
                    line_no: 1
                },
                MetaToken {
                    content: "=".to_string(),
                    token: Token::Id,
                    line_no: 1,
                },
                MetaToken {
                    content: "ok".to_string(),
                    token: Token::Id,
                    line_no: 1,
                },
                MetaToken {
                    content: "3.4".to_string(),
                    token: Token::Num(3.4),
                    line_no: 1,
                },
                MetaToken {
                    content: "1".to_string(),
                    token: Token::Num(1.0),
                    line_no: 2,
                },
                MetaToken {
                    content: "=".to_string(),
                    token: Token::Id,
                    line_no: 2,
                },
                MetaToken {
                    content: "_".to_string(),
                    token: Token::Id,
                    line_no: 2,
                },
                MetaToken {
                    content: "true".to_string(),
                    token: Token::True,
                    line_no: 2,
                },
                MetaToken {
                    content: "false".to_string(),
                    token: Token::False,
                    line_no: 2,
                },
                MetaToken {
                    content: "if".to_string(),
                    token: Token::If,
                    line_no: 2,
                },
                MetaToken {
                    content: "else".to_string(),
                    token: Token::Else,
                    line_no: 2,
                },
                MetaToken {
                    content: "true1".to_string(),
                    token: Token::Id,
                    line_no: 2,
                },
            ];
            assert_eq!(expected, r);
        },
        Err(_) => println!("Error getting the return value."),
    }
}
