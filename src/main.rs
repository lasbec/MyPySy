use std::collections::{ HashMap };

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

fn lex_keyword_base_case(prefix_tree: &PrefixTree) -> Option<Token> {
    match prefix_tree {
        PrefixTree::Node(token, _) => token.clone(),
    }
}

fn lex_keyword(prefix_tree: &PrefixTree, it: &mut impl PeekableIterator<Item = char>, content: &mut String, i: i32) -> Option<Token> {
    if let Some(c) = it.peek(){
        if let Some(child_tree) = prefix_tree.get_child(c) {
                match it.next() {
                    None => panic!("ABC 2"),
                    Some(c) => {
                        content.push(c);
                        lex_keyword(child_tree, it, content, i+1)
                    }
                }
            } else {
                lex_keyword_base_case(prefix_tree)
            }
    } else  {
        lex_keyword_base_case(prefix_tree)
    }
}

#[derive(Debug)]
enum PrefixTree {
    Node(Option<Token>, HashMap::<char, PrefixTree>)
}

impl PrefixTree {
    fn new() -> PrefixTree {
        return PrefixTree::Node(None, HashMap::new());
    }

    fn add_leaf(&mut self, c:char ,token:Token) -> &mut PrefixTree {
        let leaf = PrefixTree::Node(Some(token.clone()), HashMap::new());
        match self {
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
            PrefixTree::Node(_, map) => {
                map.get(c)
            }
        }
    }

    fn get_mut_child(&mut self,c:&char) -> Option<&mut PrefixTree> {
        match self {
            PrefixTree::Node(_, map) => {
                map.get_mut(c)
            }
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
        let mut content = String::new();
        let token = lex_keyword(&prefixMap, &mut it, &mut content, 1);
        match token {
            None => {},
            Some(t) => {
                result.push(MetaToken{
                    content,
                    token: t,
                    line_no
                });
            }
        }
        let c = if let Some(&new_c) = it.peek() {new_c}else{break;};
        match c {
            ' ' | '\t' => {
                it.next();
            },
            '\n'  =>  {
                line_no += 1;
                it.next();
            },
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
fn map_token() {
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
fn and_token() {
    let input = String::from("!= &&");
    let result = lex(&input);
    match result {
        Ok(r) => {
            let expected = vec![
                MetaToken {
                    content: "!=".to_string(),
                    token: Token::Ne,
                    line_no: 1
                },
                MetaToken {
                    content: "&&".to_string(),
                    token: Token::And,
                    line_no: 1
                }];
            assert_eq!(expected, r);
        },
        Err(_) => println!("Error getting the return value."),
    }
}

#[test]
fn identifier_token_types() {
    let input = String::from("!= hello4u");
    let result = lex(&input);
    match result {
        Ok(r) => {
            let expected = vec![
                MetaToken {
                    content: "!=".to_string(),
                    token: Token::Ne,
                    line_no: 1
            },
            MetaToken {
                content: "hello4u".to_string(),
                token: Token::Id,
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
