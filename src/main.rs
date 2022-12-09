use std::f32::consts::E;

fn main() {
    println!("Hello, Py!");
}

#[derive(PartialEq, Eq, Debug)]
enum Mark {
    Begin,
    End,
}

#[derive(PartialEq, Eq, Debug)]
enum Variable {
    Function,
    Method,
    Reference,
    Constant,
    Primitive,
}

#[derive(PartialEq, Eq, Debug)]
enum Keyword {
    Declaration(Mark, Variable),
    Initialization(Mark, Variable),
    DeclareAndInit(Mark, Variable),
    Override(Mark, Variable),
}

#[derive(PartialEq, Eq, Debug)]
struct Token {
    specific: Keyword,
    lineNo: i32,
    line: String,
}

fn tokenize(pyStr: &String) -> Vec<Token> {
    let pyLines = pyStr.split("\n");
    let mut result:Vec<Token> = Vec::new();
    let mut lineNo: i32 = 0;
    for line in pyLines {
        lineNo += 1;
        result.push(Token { lineNo, line:String::from(line), specific: Keyword::Declaration(Mark::Begin, Variable::Constant) });
    }
    return result;
}

#[test]
fn test_hello_world() {
    let inp = String::from("print('Hallo Welt')");
    let result = tokenize(&inp);
    let t = Token {
        lineNo: 1,
        line: String::from("print('Hallo Welt')"),
        specific: Keyword::Declaration(Mark::Begin, Variable::Constant) 
    };
    let mut  expected:Vec<Token>  = Vec::new();
    expected.push(t);
    assert_eq!(result, expected);
}
