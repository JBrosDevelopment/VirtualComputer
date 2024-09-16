use std::io::{self, Result};
use std::vec::Vec;
use crate::assembly::{string_to_bytes, compile_assembly_to_binary};
use crate::vc_8bit::{self, Byte};
/// # Compile
/// Compiles code to Assembly
/// # Arguments
/// * `contents` - code
/// # Returns
/// * `String` - Assembly code
/// # Example
/// ```
/// use vc_8bit::c_lang;
/// let contents = "print('a');";
/// let result = c_lang::compile(&contents);
/// ```
/// # Panics
/// This function will panic if the code is invalid
pub fn compile(contents: &String) -> String {
    let lex: Vec<Line> = get_lexer_lines(contents);
    let par = parse(lex);
    interpret(par.clone())
}

#[derive(Clone, Debug, PartialEq)] 
pub enum TokenType {
    Plus, Dash, Star, Slash, Equal, GreaterThan, LessThan, SingleQuote, Not, EqualCompare, And, Or, Identifier,
    OpenParen, CloseParen, OpenCurley, CloseCurley, Comma, NotEqual, AndAnd, OrOr, OpenBracket, CloseBracket, XOR,
    GreaterThanOrEqualTo, LessThanOrEqualTo, Number, TypeName, Statement, Boolean, ShiftLeft, ShiftRight, None, Increment, Decrement
}

#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>, pub number: i32
}
#[derive(Clone, Debug, PartialEq)] 
pub struct Token {
    pub token_type: TokenType, pub value: String
}
pub fn get_lexer_lines(contents: &str) -> Vec<Line> {
    let lines: Vec<&str> = contents.split(";").filter(|&x| x.trim() != "").collect();
    let mut lexer_lines: Vec<Line> = Vec::new();
    
    // return lexer lines
    let mut line_number = 0;
    for line in lines {
        line_number += 1;
        let l = get_lexer_line(line.trim(), line_number);
        lexer_lines.push(l);
    }
    lexer_lines
}
pub fn get_lexer_line(line: &str, line_number: i32, ) -> Line {
    let chars: Vec<&str> = line.split("").collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut single: bool = false;
    let mut skip: usize = 0;
    let mut number: String = String::new();
    let mut alphabetical: String = String::new();

    // loop through chars
    for (j, &c) in chars.iter().enumerate() {
        if skip > 0 { 
            skip -= 1;
            continue;
        }
        // is dot
        if c == "." {
            // and is number
            if number != "".to_string() {
                number += c;
            }
            // and next is number
            else if chars.len() > j + 1 && parse_str_to_i32(&chars[j + 1].to_string()).is_ok() {
                number += c;
            }
        }
        // is slash
        else if c == "/" {
            // and apart of comment
            if chars.len() > j + 1 && chars[j + 1] == "/" {
                break;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::Slash, value: c.to_string() });
                single = true;
            }
        }
        // is less than
        else if c == "<" {
            // and apart of less than or equal to
            if chars.len() > j + 1 && chars[j + 1] == "=" {
                tokens.push(Token { token_type: TokenType::LessThanOrEqualTo, value: "<=".to_string() });
                skip = 2;
            }
            else if chars.len() > j + 1 && chars[j + 1] == "<" {
                tokens.push(Token { token_type: TokenType::ShiftLeft, value: "<<".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::LessThan, value: c.to_string() });
                single = true;
            }
        }
        // is greater than
        else if c == ">" {
            if chars.len() > j + 1 && chars[j + 1] == "=" {
                tokens.push(Token { token_type: TokenType::GreaterThanOrEqualTo, value: ">=".to_string() });
                skip = 2;
            }
            else if chars.len() > j + 1 && chars[j + 1] == ">" {
                tokens.push(Token { token_type: TokenType::ShiftRight, value: ">>".to_string() });
                skip = 2;
            }
            // and by itself 
            else {
                tokens.push(Token { token_type: TokenType::GreaterThan, value: c.to_string() });
                single = true;
            }
        }
        // is equal to 
        else if c == "=" {
            if chars.len() > j + 1 && chars[j + 1] == "=" {
                tokens.push(Token { token_type: TokenType::EqualCompare, value: "==".to_string() });
                skip = 2;
            }
            else {
                tokens.push(Token { token_type: TokenType::Equal, value: c.to_string() });
                single = true;
            }
        }
        // is not
        else if c == "!" {
            if chars.len() > j + 1 && chars[j + 1] == "=" {
                tokens.push(Token { token_type: TokenType::NotEqual, value: "!=".to_string() });
                skip = 2;
            }
            else {
                tokens.push(Token { token_type: TokenType::Not, value: c.to_string() });
                single = true;
            }
        }
        else if c == "+" {
            if chars.len() > j + 1 && chars[j + 1] == "+" {
                tokens.push(Token { token_type: TokenType::Identifier, value: alphabetical.trim().to_string() });
                tokens.push(Token { token_type: TokenType::Increment, value: "++".to_string() });
                alphabetical = String::new();
                skip = 2;
            }
            else {
                tokens.push(Token { token_type: TokenType::Plus, value: "+".to_string() });
                single = true;
            }
        }
        else if c == "-" {
            if chars.len() > j + 1 && chars[j + 1] == "-" {
                tokens.push(Token { token_type: TokenType::Identifier, value: alphabetical.trim().to_string() });
                tokens.push(Token { token_type: TokenType::Decrement, value: "--".to_string() });
                alphabetical = String::new();
                skip = 2;
            }
            else {
                tokens.push(Token { token_type: TokenType::Plus, value: "+".to_string() });
                single = true;
            }
        }
        else if c == "&" {
            if chars.len() > j + 1 && chars[j + 1] == "&" {
                tokens.push(Token { token_type: TokenType::AndAnd, value: "&&".to_string() });
                skip = 2;
            }
            else {
                tokens.push(Token { token_type: TokenType::And, value: "&".to_string() });
                single = true;
            }
        }
        // is or
        else if c == "|" {
            if chars.len() > j + 1 && chars[j + 1] == "|" {
                tokens.push(Token { token_type: TokenType::OrOr, value: "||".to_string() });
                skip = 2;
            }
            else {
                tokens.push(Token { token_type: TokenType::Or, value: "|".to_string() });
                single = true;
            }
        }
        // is array
        else if c == "[" {
            if chars.len() > j + 1 && chars[j + 1] == "]" && !alphabetical.is_empty() {
                alphabetical += "[]";
                skip = 1;
            }
            else {
                tokens.push(Token { token_type: TokenType::OpenBracket, value: c.to_string() });
                single = true;
            }
        }
        // is apart of single quote
        else if c == "'" {
            if chars.len() > j + 2 && chars[j + 2] == "'" {
                let value = chars[j + 1].to_string();
                tokens.push(Token { token_type: TokenType::SingleQuote, value: (value.chars().collect::<Vec<char>>()[0]).to_string() });
                skip = 2;
            }
            else if chars.len() > j + 3 && chars[j + 1] == "\\" && chars[j + 3] == "'" {
                let mut value = chars[j + 1].to_string();
                value += chars[j + 2].to_string().as_str();
                let c = match value.chars().collect::<Vec<char>>() {
                    vec if vec.len() == 2 => match (vec[0], vec[1]) {
                        ('\\', 'n') => '\n',
                        ('\\', 't') => '\t',
                        ('\\', 'r') => '\r',
                        ('\\', '0') => '\0',
                        ('\\', '\'') => '\'',
                        ('\\', '\"') => '\"',
                        ('\\', '\\') => '\\',
                        _ => panic!("Invalid escape character"),
                    },
                    _ => panic!("Invalid escape character"),
                };
                tokens.push(Token { token_type: TokenType::SingleQuote, value: c.to_string() });
                skip = 3; 
            }
            else {
                panic!("Expected char quote to be one character");
            }
        }
        // symbols:
        else if c == "]" { tokens.push(Token { token_type: TokenType::CloseBracket, value: c.to_string() }); single = true; }
        else if c == "+" { tokens.push(Token { token_type: TokenType::Plus, value: c.to_string() }); single = true; }
        else if c == "-" { tokens.push(Token { token_type: TokenType::Dash, value: c.to_string() }); single = true; }
        else if c == "*" { tokens.push(Token { token_type: TokenType::Star, value: c.to_string() }); single = true; }
        else if c == "(" { tokens.push(Token { token_type: TokenType::OpenParen, value: c.to_string() }); single = true; }
        else if c == ")" { tokens.push(Token { token_type: TokenType::CloseParen, value: c.to_string() }); single = true; }
        else if c == "{" { tokens.push(Token { token_type: TokenType::OpenCurley, value: c.to_string() }); single = true; }
        else if c == "}" { tokens.push(Token { token_type: TokenType::CloseCurley, value: c.to_string() }); single = true; }
        else if c == "," { tokens.push(Token { token_type: TokenType::Comma, value: c.to_string() }); single = true; }
        else if c == "^" { tokens.push(Token { token_type: TokenType::XOR, value: c.to_string() }); single = true; }
        // is alphabetical
        else if is_alphabetical(c) || !alphabetical.is_empty() {
            alphabetical += c;
        }
        // is number
        else if let Ok(_) = parse_str_to_i32(c) {
            number += c;
        }
        // spaces:
        if c == " " || chars.len() == j + 1 || single {
            let mut tok: Token = Token { token_type:TokenType::None, value:String::new() };
            if single {
                tok = tokens.last().unwrap().clone();
                tokens.remove(tokens.len() - 1);
            }
            if !number.is_empty() {
                tokens.push(Token { token_type: TokenType::Number, value: number.to_string() });
                number = String::new();
            }
            if !alphabetical.trim().is_empty() {
                match alphabetical.trim() {
                    "uint8" | "char" | "bool" => tokens.push(Token { token_type: TokenType::TypeName, value: alphabetical.trim().to_string() }),
                    "uint8[]" | "char[]" | "bool[]" => tokens.push(Token { token_type: TokenType::TypeName, value: alphabetical.trim().to_string() }),
                    "if" | "while" => tokens.push(Token { token_type: TokenType::Statement, value: alphabetical.trim().to_string() }),
                    "true" | "false" => tokens.push(Token { token_type: TokenType::Boolean, value: alphabetical.trim().to_string() }),
                    _ => tokens.push(Token { token_type: TokenType::Identifier, value: alphabetical.trim().to_string() })
                }
                alphabetical = String::new();
            }
            if single {
                single = false;
                tokens.push(tok);
            }
        }
    }

    Line { tokens:tokens, number:line_number }
}
fn parse_str_to_i32(s: &str) -> Result<i32> {
    match s.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}
fn is_alphabetical(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if s.len() == 1 {
        match s.parse::<char>() {
            Ok(c) => return c.is_alphabetic(),
            Err(_) => return false
        };
    }
    else {
        return false;
    }
}
#[derive(Debug, Clone)]
pub struct ExprNode {
    pub c: String,
    pub token: Token,
    pub line: i32,
    pub operand1: Option<Box<ExprNode>>,
    pub operand2: Option<Box<ExprNode>>,
    pub func_parameters: Option<Vec<Vec<Option<ExprNode>>>>,
    pub func_name: Option<String>,
    pub set: Option<Vec<Vec<Option<ExprNode>>>>,
    pub bracket_set: Option<Vec<Option<ExprNode>>>,
    pub statement_insides: Option<Vec<Option<ExprNode>>>,
    pub statement_lines: Option<Vec<ExprNode>>
}

impl ExprNode {
    fn is_func(&self) -> bool {
        self.func_name.is_some() && self.func_parameters.is_some() 
    }
    fn new_num(num: Token, line: i32) -> Self {
        ExprNode {
            c: num.value.clone(),
            token: num,
            line: line,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            set: None,
            bracket_set: None,
            statement_insides: None,
            statement_lines: None
        }
    }
    fn new_op(op: Token, e1: ExprNode, e2: ExprNode, line: i32) -> Self {
        ExprNode {
            c: op.value.clone(),
            token: op,
            line: line,
            operand1: Some(Box::new(e1)),
            operand2: Some(Box::new(e2)),
            func_parameters: None,
            func_name: None,
            set: None,
            bracket_set: None,
            statement_insides: None,
            statement_lines: None
        }
    }
    fn new_func(func: Token, parameters: Option<Vec<Vec<Option<ExprNode>>>>, line: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: func.clone(),
            line: line,
            operand1: None,
            operand2: None,
            func_parameters: parameters,
            func_name: Some(func.value.clone()),
            set: None,
            bracket_set: None,
            statement_insides: None,
            statement_lines: None
        }
    }
    fn new_set(token: Token, indexes: Option<Vec<Vec<Option<ExprNode>>>>, line_num: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: token.clone(),
            line: line_num,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            set: indexes.clone(),
            bracket_set: None,
            statement_insides: None,
            statement_lines: None
        }
    }
    fn new_bracket_set(token: Token, indexes: Option<Vec<Option<ExprNode>>>, line_num: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: token.clone(),
            line: line_num,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            set: None,
            bracket_set: indexes.clone(),
            statement_insides: None,
            statement_lines: None
        }
    }
    fn new_statement(token: Token, insides: Vec<Option<ExprNode>>, lines: Vec<ExprNode>, line_num: i32) -> Self {
        ExprNode {
            c: String::new(),
            token: token.clone(),
            line: line_num,
            operand1: None,
            operand2: None,
            func_parameters: None,
            func_name: None,
            set: None,
            bracket_set: None,
            statement_insides: Some(insides),
            statement_lines: Some(lines)
        }
    }
}

pub fn precedence(op: String) -> i32 {
    match op.as_str() {
        "=" => 1,
        "&&" | "||" => 2,
        "&" | "|" | ">>" | "<<" => 3,
        "==" | "!=" => 4,
        ">" | "<" | ">=" | "<=" => 5,
        "+" | "-" => 6,
        "*" | "/" => 7,
        "!" => 8,
        _ => 0,
    }
}
pub fn parse(lexer_lines: Vec<Line>) -> Vec<Option<ExprNode>> {
    let mut lines = lexer_lines.iter().peekable();
    let mut returns: Vec<Option<ExprNode>> = Vec::new();

    while let Some(l) = lines.peek() {
        let mut chars = l.tokens.iter().peekable();
        let mut last_was_digit_or_closing = false;
        let mut last_was_variable = false;
        let mut operator_stack: Vec<Token> = Vec::new();
        let mut expr_stack: Vec<ExprNode> = Vec::new();
        let number = l.number;

        while let Some(c) = chars.peek() {
            match c.token_type {
                TokenType::OpenParen => {
                    if last_was_digit_or_closing || last_was_variable {
                        // Implicit multiplication: e.g., "2(" or ")("
                        operator_stack.push(Token { value: "*".to_string(), token_type: TokenType::Star });
                    }
                    operator_stack.push((*c).clone());
                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::CloseParen => {
                    while operator_stack.last().map_or(false, |op| op.token_type != TokenType::OpenParen) && expr_stack.len() > 1 {
                        let operator = operator_stack.pop().unwrap();
                        let e2 = expr_stack.pop().unwrap();
                        let e1 = expr_stack.pop().unwrap();
                        expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                    }
                    operator_stack.pop(); // Pop the '('
                    chars.next();
                    last_was_digit_or_closing = true;
                    last_was_variable = false;
                }
                TokenType::Number | TokenType::Boolean => {
                    expr_stack.push(ExprNode::new_num((*c).clone(), number));
                    chars.next();
                    last_was_digit_or_closing = true;
                    last_was_variable = false;
                }
                TokenType::SingleQuote => {
                    expr_stack.push(ExprNode::new_num((*c).clone(), number));
                    chars.next();
                    last_was_digit_or_closing = true;
                    last_was_variable = false;
                }
                TokenType::Dash => {
                    if !last_was_digit_or_closing && !last_was_variable {
                        chars.next();
                        let operator = Token { value: "-".to_string(), token_type: TokenType::Dash };
                        let e1 = ExprNode::new_num(Token { token_type: TokenType::Number, value: "0".to_string() }, number);
                        let e2 = ExprNode::new_num(chars.peek().unwrap().clone().clone(), number);
                        expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                    } else {
                        while operator_stack.last().map_or(false, |top| precedence(top.value.clone()) >= precedence(c.value.clone())) && expr_stack.len() > 1 {
                            let operator = operator_stack.pop().unwrap();
                            let e2 = expr_stack.pop().unwrap();
                            let e1 = expr_stack.pop().unwrap();
                            expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                        }
                        operator_stack.push((*c).clone());
                    }
                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Not => {
                    chars.next();
                    let operator = Token { value: "!".to_string(), token_type: TokenType::Not };
                    let e1 = ExprNode::new_num(chars.peek().unwrap().clone().clone(), number);
                    let e2 = ExprNode::new_num(Token { token_type: TokenType::None, value: "".to_string() }, number);
                    expr_stack.push(ExprNode::new_op(operator, e1, e2, number));

                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Increment => {
                    let operator = Token { value: "++".to_string(), token_type: TokenType::Increment };
                    let e1 = expr_stack.pop().unwrap();
                    let e2 = ExprNode::new_num(Token { token_type: TokenType::None, value: String::new() }, number);
                    expr_stack.push(ExprNode::new_op(operator, e1, e2, number));

                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Decrement => {
                    let operator = Token { value: "--".to_string(), token_type: TokenType::Decrement };
                    let e1 = expr_stack.pop().unwrap();
                    let e2 = ExprNode::new_num(Token { token_type: TokenType::None, value: String::new() }, number);
                    expr_stack.push(ExprNode::new_op(operator, e1, e2, number));

                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Plus | TokenType::Star | TokenType::Slash | TokenType::GreaterThan | TokenType::GreaterThanOrEqualTo | TokenType::AndAnd | TokenType::EqualCompare | TokenType::XOR |
                TokenType::LessThan | TokenType::LessThanOrEqualTo | TokenType::Comma | TokenType::NotEqual | TokenType::Equal | TokenType::OrOr | TokenType::Or | TokenType::And | TokenType::ShiftLeft | TokenType::ShiftRight => {
                    while operator_stack.last().map_or(false, |top| precedence(top.value.clone()) >= precedence(c.value.clone())) && expr_stack.len() > 1 {
                        let operator = operator_stack.pop().unwrap();
                        let e2 = expr_stack.pop().unwrap();
                        let e1 = expr_stack.pop().unwrap();
                        expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
                    }
                    operator_stack.push((*c).clone());
                    chars.next();
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::TypeName => {
                    let operator = Token { value: format!("{}", chars.next().unwrap().value.to_string()), token_type: TokenType::TypeName };
                    let e1 = ExprNode::new_num(chars.next().unwrap().clone(), number);
                    let e2 = ExprNode::new_num(Token { token_type: TokenType::None, value: String::new() }, number);
                    expr_stack.push(ExprNode::new_op(operator, e1, e2, number));

                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Statement => {
                    // Get all tokens inside the parentheses
                    let mut tokens_inside_parenthesis: Vec<Token> = vec![];
                    let mut parenthesis_count = 0;
                    let name = (*c).clone();
                    chars.next();

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenParen => {
                                parenthesis_count += 1;
                                if parenthesis_count > 1 {
                                    tokens_inside_parenthesis.push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseParen => {
                                parenthesis_count -= 1;
                                let mut breaks = false;
                                if parenthesis_count == 0 {
                                    breaks = true;
                                }
                                else {
                                    tokens_inside_parenthesis.push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            _ => {
                                tokens_inside_parenthesis.push((*c).clone());
                                chars.next();
                            }
                        }
                    }
                    
                    let mut parsed_tokens: Vec<Option<ExprNode>> = Vec::new();
                    if tokens_inside_parenthesis.len() != 0 {   
                        parsed_tokens = parse(vec![Line { number: l.number, tokens: tokens_inside_parenthesis }]);
                    }
                    
                    let mut curley_tokens_inside_parens: Vec<Vec<Token>> = vec![vec![]];
                    let mut curley_index = 0;
                    let mut curley_count = 0;

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenCurley => {
                                curley_count += 1;
                                if curley_count > 1 {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseCurley => {
                                curley_count -= 1;
                                let mut breaks = false;
                                if curley_count <= 0 {
                                    breaks = true;
                                }
                                else {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            TokenType::Comma => {
                                if curley_count == 1 {
                                    curley_tokens_inside_parens.push(vec![]);
                                    curley_index += 1;
                                }
                                else {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            _ => {
                                curley_tokens_inside_parens[curley_index].push((*c).clone());
                                chars.next();
                            }
                        }
                    }

                    let mut curley_parsed_tokens: Vec<Vec<Option<ExprNode>>> = Vec::new();
                    if curley_tokens_inside_parens[0].len() != 0 {   
                        for tokens_inside_parens in curley_tokens_inside_parens {
                            curley_parsed_tokens.push(parse(vec![Line { number: l.number, tokens: tokens_inside_parens }]));
                        }
                    }
                    
                    let lines: Vec<ExprNode> = curley_parsed_tokens.iter().map(|x| x[0].clone().unwrap()).collect();

                    expr_stack.push(ExprNode::new_statement(name, parsed_tokens, lines, number));

                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::Identifier => {
                    // Get all tokens inside the parentheses
                    let mut params_tokens_inside_parens: Vec<Vec<Token>> = vec![vec![]];
                    let mut params_index = 0;
                    let mut parenthesis_count = 0;
                    let mut index = 0;
                    let name = (*c).clone();
                    chars.next();

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenParen => {
                                index += 1;
                                parenthesis_count += 1;
                                if parenthesis_count > 1 {
                                    params_tokens_inside_parens[params_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseParen => {
                                if index == 0 {
                                    break;
                                }
                                index += 1;
                                parenthesis_count -= 1;
                                let mut breaks = false;
                                if parenthesis_count == 0 {
                                    breaks = true;
                                }
                                else {
                                    params_tokens_inside_parens[params_index].push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            TokenType::Comma => {
                                if index == 0 {
                                    break;
                                }
                                index += 1;
                                if parenthesis_count == 1 {
                                    params_tokens_inside_parens.push(vec![]);
                                    params_index += 1;
                                }
                                else {
                                    params_tokens_inside_parens[params_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            _ => {
                                if index == 0 {
                                    break;
                                }
                                index += 1;
                                params_tokens_inside_parens[params_index].push((*c).clone());
                                chars.next();
                            }
                        }
                    }

                    if index == 0 {
                        // is constant
                        if last_was_digit_or_closing || last_was_variable {
                            operator_stack.push(Token { value: "*".to_string(), token_type: TokenType::Star });
                        }

                        expr_stack.push(ExprNode::new_num(name.clone(), number));
                        last_was_digit_or_closing = true;
                        last_was_variable = true;
                    }
                    else {
                        // is function
                        let mut parsed_tokens: Vec<Vec<Option<ExprNode>>> = Vec::new();
                        if params_tokens_inside_parens[0].len() != 0 {   
                            for tokens_inside_parens in params_tokens_inside_parens {
                                parsed_tokens.push(parse(vec![Line { number: l.number, tokens: tokens_inside_parens }]));
                            }
                        }
                        
                        expr_stack.push(ExprNode::new_func(name, Some(parsed_tokens), number));
                        
                        last_was_digit_or_closing = true;
                        last_was_variable = false;
                    }
                }
                TokenType::OpenCurley => {
                    // Get all tokens inside the curleys
                    let mut curley_tokens_inside_parens: Vec<Vec<Token>> = vec![vec![]];
                    let mut curley_index = 0;
                    let mut curley_count = 1;
                    let token = (*c).clone();
                    chars.next();

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenCurley => {
                                curley_count += 1;
                                if curley_count > 1 {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseCurley => {
                                curley_count -= 1;
                                let mut breaks = false;
                                if curley_count == 0 {
                                    breaks = true;
                                }
                                else {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            TokenType::Comma => {
                                if curley_count == 1 {
                                    curley_tokens_inside_parens.push(vec![]);
                                    curley_index += 1;
                                }
                                else {
                                    curley_tokens_inside_parens[curley_index].push((*c).clone());
                                }
                                chars.next();
                            }
                            _ => {
                                curley_tokens_inside_parens[curley_index].push((*c).clone());
                                chars.next();
                            }
                        }
                    }

                    let mut parsed_tokens: Vec<Vec<Option<ExprNode>>> = Vec::new();
                    if curley_tokens_inside_parens[0].len() != 0 {
                        for tokens_inside_parens in curley_tokens_inside_parens {
                            parsed_tokens.push(parse(vec![Line { number: l.number, tokens: tokens_inside_parens }]));
                        }
                    }
                    
                    expr_stack.push(ExprNode::new_set(token, Some(parsed_tokens), number));
                    
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::CloseCurley => {
                    panic!("Unexpected character '{}', has '}}' without opening '{{' in line {}", c.value, number); 
                }
                TokenType::OpenBracket => {
                    // Get all tokens inside the brackets
                    let mut bracket_tokens_inside_parens: Vec<Token> = vec![];
                    let mut bracket_count = 1;
                    let token = (*c).clone();
                    chars.next();

                    while let Some(c) = chars.peek() {
                        match c.token_type {
                            TokenType::OpenBracket => {
                                bracket_count += 1;
                                if bracket_count > 1 {
                                    bracket_tokens_inside_parens.push((*c).clone());
                                }
                                chars.next();
                            }
                            TokenType::CloseBracket => {
                                bracket_count -= 1;
                                let mut breaks = false;
                                if bracket_count == 0 {
                                    breaks = true;
                                }
                                else {
                                    bracket_tokens_inside_parens.push((*c).clone());
                                }
                                chars.next();
                                
                                if breaks{
                                    break;
                                }
                            }
                            _ => {
                                bracket_tokens_inside_parens.push((*c).clone());
                                chars.next();
                            }
                        }
                    }

                    let mut parsed_tokens: Vec<Option<ExprNode>> = Vec::new();
                    if bracket_tokens_inside_parens.len() != 0 {
                        parsed_tokens = parse(vec![Line { number: l.number, tokens: bracket_tokens_inside_parens }]);
                    }
                    
                    let mut node = ExprNode::new_bracket_set(token, Some(parsed_tokens), number);
                    node.operand1 = Some(Box::new(expr_stack.pop().unwrap()));
                    expr_stack.push(node);
                    
                    last_was_digit_or_closing = false;
                    last_was_variable = false;
                }
                TokenType::CloseBracket => {
                    panic!("Unexpected character '{}', has ']' without opening '[' in line {}", c.value, number); 
                }
                _ => {
                    panic!("Unexpected token '{}' in line {}", c.value, number); // Error handling
                }
            }
        }

        let mut i = 0;
        while let Some(operator) = operator_stack.pop() {
            i += 1;
            if i < 99 {
                let mut e1 = ExprNode::new_num(Token { value: String::new(), token_type: TokenType::None }, number);
                let mut e2 = ExprNode::new_num(Token { value: String::new(), token_type: TokenType::None }, number);
                if !expr_stack.is_empty() {
                    e2 = expr_stack.pop().unwrap();
                }
                if !expr_stack.is_empty() {
                    e1 = expr_stack.pop().unwrap();
                }
                expr_stack.push(ExprNode::new_op(operator, e1, e2, number));
            }
        }

        returns.push(expr_stack.pop());
        lines.next();
    }
    returns.iter().filter(|x| x.is_some()).map(|x| x.clone()).collect::<Vec<Option<ExprNode>>>()
}

pub fn fmt_expr(node: &ExprNode) -> String {
    if node.func_name.is_some() && node.func_parameters.is_some() {
        let mut func_params: String = String::new();
        for n in node.func_parameters.as_ref().unwrap() {
            for o in n.iter() {
                if let Some(p) = o.as_ref() {
                    func_params = format!("{}, {}", func_params, fmt_expr(p));
                }
            }
        }

        return format!("{}({})", node.func_name.as_ref().unwrap(), func_params.trim_start_matches(", "));
    }

    if node.set.is_some() {
        let mut set_curley: String = String::new();
        for n in node.set.as_ref().unwrap() {
            for o in n.iter() {
                if let Some(p) = o.as_ref() {
                    set_curley = format!("{}, {}", set_curley, fmt_expr(p));
                }
            }
        }
        
        return format!("{{ {} }}", set_curley.trim_start_matches(", "));
    }
    if node.bracket_set.is_some() {
        let mut set_braces: String = String::new();
        for n in node.bracket_set.as_ref().unwrap() {
            if let Some(p) = n.as_ref() {
                set_braces = format!("{}, {}", set_braces, fmt_expr(p));
            }
        }
        
        return format!("{}[{}]", node.operand1.as_ref().unwrap().token.value.clone(), set_braces.trim_start_matches(", "));
    }

    if node.statement_insides.is_some() || node.statement_lines.is_some() {
        let st = fmt_expr(node.statement_insides.clone().unwrap()[0].as_ref().unwrap());
        let set = fmt_expr(Some(ExprNode {
            token: node.token.clone(),
            line: node.line.clone(),
            c: node.c.clone(),
            func_name: None,
            func_parameters: None,
            operand1: None,
            operand2: None,
            set: Some(vec![node.statement_lines.clone().unwrap().iter().map(|n| Some(n.clone())).collect::<Vec<_>>()]),
            bracket_set: None,
            statement_insides: None,
            statement_lines: None,
        }).as_ref().unwrap());
        
        return format!("{} ({}) {}", node.token.value, st, set);
    }
        
    if node.operand1.is_none() && node.operand2.is_none() {
        if node.token.token_type == TokenType::SingleQuote {
            return format!("'{}'", node.c);
        }
        return node.c.to_string();
    }

    let left = node.operand1.as_ref().map_or("".to_string(), |n| fmt_expr(n));
    let right = node.operand2.as_ref().map_or("".to_string(), |n| fmt_expr(n));

    format!("({}{}{})", left, node.c, right)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Bool, Char, UInt8, None
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub address: Byte,
    pub var_type: VariableType,
    pub is_array: bool
}

impl Variable {
    pub fn new(name: String, address: Byte, var_type: VariableType, is_array: bool) -> Self {
        Variable {
            name,
            address,
            var_type,
            is_array
        }
    }
}

pub fn interpret(lines: Vec<Option<ExprNode>>) -> String {
    let mut variables: Vec<Variable> = vec![];
    let lines_binding = lines.clone();
    let lines= lines_binding.iter().cloned();
    let mut result = String::new();
    let mut bytes = 0;

    let mut virtual_registers: [Byte; 4] = [Byte::zero(), Byte::zero(), Byte::zero(), Byte::zero()];
    for (_, line) in lines.enumerate() {
        if line.is_none() {
            continue;
        }
        if let Some(line) = line {
            //result += format!("; {}\n", line.token.value).as_str();
            let contents = solve_node(&line, &mut variables, "R0", &mut virtual_registers, VariableType::None, &mut bytes);
            result += contents.clone().as_str().trim();
            
            let assembly_to_binary = &compile_assembly_to_binary(contents.as_str());
            let bytes_vec = string_to_bytes(assembly_to_binary);
            bytes += bytes_vec.len() as i32;
            result += format!(" ; BYTE ADDRESS {bytes}\n").as_str();
        }
    }

    result += "HALT";
    result
}
pub fn solve_node(node: &ExprNode, variables: &mut Vec<Variable>, register: &str, virtual_registers: &mut [Byte; 4], expected_value: VariableType, bytes: &mut i32) -> String {
    match node.token.token_type {
        TokenType::Equal => {
            if node.operand1.clone().unwrap().token.token_type == TokenType::TypeName {
                let var_type = node.operand1.as_ref().unwrap();
                let var_name = var_type.operand1.as_ref().unwrap();

                let v_type = match var_type.token.value.clone().as_str() {
                    "bool" | "bool[]" => VariableType::Bool,
                    "char" | "char[]" => VariableType::Char,
                    "uint8" | "uint8[]" => VariableType::UInt8,
                    _ => panic!("Invalid variable type")
                };

                // Handle array type declaration
                if var_type.operand2.clone().is_some() && var_type.token.clone().value.ends_with("[]") {

                    // Parse the array values
                    if let Some(array_values_node) = node.operand2.clone() {
                        let array_values = (*array_values_node).set.unwrap();

                        // Allocate space for the array in memory
                        let mut assembly = String::new();
                        for (i, value_node) in array_values.iter().enumerate() {
                            let value = solve_node(value_node.iter().nth(0).unwrap().as_ref().unwrap(), variables, "R0", virtual_registers, v_type, bytes);
                            let address = (crate::vc_8bit::MAXBYTE - (variables.len() as i32 + 1)).try_into().unwrap();
                            variables.push(Variable::new(format!("{}|{}", var_name.token.value, i), address, v_type.clone(), true));
                            assembly += format!("{}\nSTR R0 #{} ; store array element {}\n", value, address.to_string(), i).as_str();
                        }

                        return assembly;
                    } else {
                        panic!("Array must have initialization values");
                    }
                }

                let value = solve_node(node.operand2.as_ref().unwrap(), variables, "R0", virtual_registers, v_type, bytes);
                let address = (crate::vc_8bit::MAXBYTE - (variables.len() as i32 + 1)).try_into().unwrap();
                variables.push(Variable::new(var_name.token.value.clone(), address, v_type, false));
                virtual_registers[0] = address;
                format!("{}\nSTR R0 #{} ; store created variable", value, address.to_string())
            } else {
                // Non-type assignments
                let var_name = node.operand1.as_ref().unwrap();
                let var_type = variables[variables.iter().position(|x| x.name == var_name.token.value).unwrap()].var_type;
                let value = solve_node(node.operand2.as_ref().unwrap(), variables, "R0", virtual_registers, var_type, bytes);
                let address = variables[variables.iter().position(|x| x.name == var_name.token.value).unwrap()].address;
                
                virtual_registers[0] = address;
                format!("{}\nSTR R0 #{} ; store variable", value, address.to_string())
            }
        }
        TokenType::AndAnd => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R3", virtual_registers, expected_value, bytes);

            format!("{} ; get value left\n{} ; get value right\nAND {} R3 ; and value '&&'", value_left, value_right, register)
        }
        TokenType::OrOr => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R3", virtual_registers, expected_value, bytes);
    
            format!("{} ; get value left\n{} ; get value right\nOR {} R3 ; or value '||'", value_left, value_right, register)
        }
        TokenType::Increment => {
            let value = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            format!("{} ; get value\nINC {} ; increment", value, register)
        }
        TokenType::Decrement => {
            let value = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            format!("{} ; get value\nDEC {} ; increment", value, register)
        }
        TokenType::Not => {
            let value = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);

            format!("{} ; get value\nNOT {} ; not value", value, register)
        }
        TokenType::And => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R3", virtual_registers, expected_value, bytes);

            format!("{} ; get value left\n{} ; get value right\nAND {} R3 ; and value", value_left, value_right, register)
        }
        TokenType::Or => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R3", virtual_registers, expected_value, bytes);

            format!("{} ; get value left\n{} ; get value right\nOR {} R3 ; or value", value_left, value_right, register)
        }
        TokenType::XOR => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R3", virtual_registers, expected_value, bytes);

            format!("{} ; get value left\n{} ; get value right\nXOR {} R3 ; xor value", value_left, value_right, register)
        }
        TokenType::ShiftLeft | TokenType::ShiftRight => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, register, virtual_registers, expected_value, bytes);
            let op2 = node.operand2.clone().unwrap().token;
            let binary: String = if op2.token_type == TokenType::Number {
                Byte::from_string(op2.value.clone()).to_string()
            } else {
                panic!("Shifting requires constant value in line {}", node.line);
            };

            if node.token.token_type == TokenType::ShiftLeft {
                format!("{} ; get value left\nSHL {} #{}", value_left, register, binary)
            }
            else {
                format!("{} ; get value left\nSHR {} #{}", value_left, register, binary)
            }
        }
        TokenType::EqualCompare | TokenType::NotEqual | TokenType::GreaterThan | TokenType::GreaterThanOrEqualTo | TokenType::LessThan |TokenType::LessThanOrEqualTo => {
            let virtual_registers_clone: [Byte; 4] = *virtual_registers;
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, "R0", virtual_registers, VariableType::None, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R1", virtual_registers, VariableType::None, bytes);
            let mut value = String::new();
            let virtual_registers_clone_2: [Byte; 4] = *virtual_registers;
            *virtual_registers = virtual_registers_clone;
            
            let left_byte = value_left.split(" ").collect::<Vec<&str>>()[2].to_string().chars().skip(1).collect::<String>();
            value += format!("{} ; value left\n", value_left).as_str();
            virtual_registers[0] = Byte::from_string(left_byte.clone());
            
            let right_byte = value_right.split(" ").collect::<Vec<&str>>()[2].to_string().chars().skip(1).collect::<String>();
            value += format!("{} ; value right\n", value_right).as_str();
            virtual_registers[1] = Byte::from_string(right_byte.clone());

            let logic = match node.token.token_type {
                TokenType::EqualCompare => "SUB R0 R1\nCMP_ZRO R0",
                TokenType::NotEqual => "SUB R0 R1\nCMP_ZRO R0\nNOT R0",
                TokenType::GreaterThan => "SUB R0 R1\nCMP_ABV R0",
                TokenType::GreaterThanOrEqualTo => "SUB R0 R1\nCMP_ABV R0\nCMP_ZRO R1\nOR R0 R1",
                TokenType::LessThan => "SUB R0 R1\nCMP_NEG R0",
                TokenType::LessThanOrEqualTo => "SUB R0 R1\nCMP_NEG R0\nCMP_ZRO R1\nOR R0 R1",
                _ => panic!("Invalid logic operator")
            };

            *virtual_registers = virtual_registers_clone_2;

            if register == "R0" {
                format!("{}{} ; compare", value, logic)
            }
            else {
                format!("{}{} ; compare\nCPY {} R0 ; copy value right", value, logic, register)
            }
        }
        TokenType::Boolean => {
            if expected_value == VariableType::Bool || expected_value == VariableType::None {
                virtual_registers[get_index_from_register(register)] = if node.token.value == "true" { Byte::full() } else { Byte::zero() };
                format!("MOV {} #{}", register, virtual_registers[get_index_from_register(register)].to_string())
            } else {
                panic!("Invalid type for boolean")
            }
        }
        TokenType::SingleQuote => {
            if expected_value == VariableType::Char || expected_value == VariableType::None {
                let c = node.token.value.clone().chars().next().unwrap();
                let ascii_value = c as u8;
                virtual_registers[get_index_from_register(register)] = Byte::from_u8(ascii_value);
                format!("MOV {} #{}", register, Byte::from_u8(ascii_value).to_string())
            } else {
                panic!("Invalid type for number")
            }
        }
        TokenType::Number => {
            if expected_value == VariableType::UInt8 || expected_value == VariableType::None {
                virtual_registers[get_index_from_register(register)] = Byte::from_string(node.token.value.clone());
                format!("MOV {} #{}", register, virtual_registers[get_index_from_register(register)].to_string())
            } else {
                panic!("Invalid type for number")
            }
        } 
        TokenType::Plus | TokenType::Dash | TokenType::Star | TokenType::Slash => {
            let value_left = solve_node(node.operand1.as_ref().unwrap(), variables, "R0", virtual_registers, VariableType::UInt8, bytes);
            let value_right = solve_node(node.operand2.as_ref().unwrap(), variables, "R1", virtual_registers, VariableType::UInt8, bytes);
            let mut value = String::new();
            
            let left_byte = value_left.split(" ").collect::<Vec<&str>>()[2].to_string().chars().skip(1).collect::<String>();
            value += format!("{} ; value left\n", value_left).as_str();
            virtual_registers[0] = Byte::from_string(left_byte.clone());
            
            let right_byte = value_right.split(" ").collect::<Vec<&str>>()[2].to_string().chars().skip(1).collect::<String>();
            value += format!("{} ; value right\n", value_right).as_str();
            virtual_registers[1] = Byte::from_string(right_byte.clone());

            let math = match node.token.token_type {
                TokenType::Plus => "ADD",
                TokenType::Dash => "SUB",
                TokenType::Star => "MUL",
                TokenType::Slash => "DIV",
                _ => panic!("Invalid math operator")
            };

            if register == "R0" {
                format!("{}{} R0 R1 ; math", value, math)
            }
            else {
                format!("{}{} R0 R1 ; math\nCPY {} R0 ; copy value right", value, math, register)
            }
        }
        TokenType::Identifier => {
            if !node.is_func() {
                for var in variables {
                    if &var.name == &node.token.value {
                        if expected_value != var.var_type && expected_value != VariableType::None {
                            panic!("Invalid type for variable '{}'", var.name);
                        }
                        virtual_registers[get_index_from_register(register)] = var.address;
                        return format!("LDR {} #{} ; load variable", register, var.address.to_string());
                    }
                }
                panic!("Variable '{}' does not exist in line {}", &node.token.value, node.line);
            }
            let func_name = node.func_name.clone().unwrap();
            if func_name == "print" {
                let v = node.func_parameters.clone().unwrap()[0][0].clone().unwrap();
                let value = solve_node(&v.clone(), variables, "R2", virtual_registers, VariableType::Char, bytes);
                let byte_value = value.split(" ").collect::<Vec<&str>>()[2].to_string().chars().skip(1).collect::<String>();
                
                virtual_registers[2] = Byte::from_string(byte_value);
                format!("{}\nMSG R2 ; print value", value)
            }
            else if func_name == "out" {
                let v = node.func_parameters.clone().unwrap()[0][0].clone().unwrap();
                let value = solve_node(&v.clone(), variables, "R2", virtual_registers, VariableType::None, bytes);

                format!("{}\nOUT R2 ; print value as byte", value)
            }
            else if func_name == "to_char" {                
                let v = node.func_parameters.clone().unwrap()[0][0].clone().unwrap();
                let value = solve_node(&v.clone(), variables, "R3", virtual_registers, VariableType::UInt8, bytes);

                let before = format!("{}\n", value);

                if register == "R3" {
                    format!("{}MOV R2 48 ; 48 is ascii '0'\nADD R3 R2 ; get ascii", before)
                }
                else {
                    format!("{}MOV R2 48 ; 48 is ascii '0'\nADD R3 R2 ; get ascii\nCPY {} R3 ; move value to correct register", before, register)
                }
            }
            else if func_name == "write_port" {                
                let v2 = node.func_parameters.clone().unwrap()[0][0].clone().unwrap();
                let value2: String = if v2.token.token_type == TokenType::Number {
                    Byte::from_string(v2.token.value.clone()).to_string()
                } else {
                    panic!("Shifting requires constant value in line {}", node.line);
                };
                let v1 = node.func_parameters.clone().unwrap()[1][0].clone().unwrap();
                let value1 = solve_node(&v1.clone(), variables, "R2", virtual_registers, VariableType::None, bytes);

                format!("{value1}\nWPRT R2 #{value2}")
            }
            else if func_name == "read_port" {                
                let v1 = node.func_parameters.clone().unwrap()[0][0].clone().unwrap();
                let value1: String = if v1.token.token_type == TokenType::Number {
                    Byte::from_string(v1.token.value.clone()).to_string()
                } else {
                    panic!("Shifting requires constant value in line {}", node.line);
                };

                format!("RPRT {register} #{value1}")
            }
            else {
                panic!("Function '{}' not defined in line {}", func_name, node.line);
            }
        }
        TokenType::OpenBracket => {
            let value = solve_node(node.bracket_set.as_ref().unwrap().iter().nth(0).unwrap().as_ref().unwrap(), variables, "R3", virtual_registers, VariableType::None, bytes);
            let name = node.operand1.as_ref().unwrap().token.value.clone();
            let func = |v_name: String, name: &str| -> bool {
                let v = v_name.split("|").collect::<Vec<&str>>()[0];
                v == name
            };

            if !variables.iter().any(|v| func(v.name.clone(), &name)) {
                panic!("Variable '{}' does not exist in line {}", name, node.line);
            }
            
            let variable = variables.iter().find(|v| func(v.name.clone(), &name)).unwrap();
            
            if !variable.is_array {
                panic!("Variable '{}' is not an array in line {}", name, node.line);
            }
            
            let byte = value.split(" ").collect::<Vec<&str>>()[2].to_string().chars().skip(1).collect::<String>();
            let index = Byte::from_string(byte.clone()).reverse().to_u8();
            let address = variables.iter().find(|v| v.name == format!("{}|{}", name, index)).unwrap().address.to_string();

            format!("{value} ; get value for array\nLDR {register} #{address}")
        }
        TokenType::Statement => {
            let value = solve_node(node.statement_insides.as_ref().unwrap().iter().nth(0).unwrap().as_ref().unwrap(), variables, "R3", virtual_registers, expected_value, bytes);
            let check = format!("{value} ; get value for statement\nMOV R2 0 ; set R2 to 0\nSUB R2 R3 ; check if statement is true");
            let mut lines = String::new();
            for i in 0..node.statement_lines.as_ref().unwrap().len() {
                lines += format!("{}\n", solve_node(node.statement_lines.as_ref().unwrap().iter().nth(i).unwrap(), variables, register, virtual_registers, expected_value, bytes)).as_str();
            }
            let assembly_to_binary = &compile_assembly_to_binary(format!("{}\n{}", check.clone(), lines).as_str());
            let bytes_vec = string_to_bytes(assembly_to_binary);
            let end_bytes = bytes_vec.len() as i32 + *bytes + 2;
            
            if node.token.value == "if" {
                let jump = format!("{check}\nJMP_ZRO {} ; jump if false", end_bytes);
                format!("{jump}\n{lines}")
            }
            else if node.token.value == "while" {
                let jump = format!("{check}\nJMP_ZRO {} ; jump if false", end_bytes + 1);
                format!("{jump}\n{lines}\nJMP {bytes} ; jump back to start")
            }
            else {
                panic!("Unsupported statement: {:?} in line {}", node.token.token_type, node.line);
            }
        }
        _ => {
            panic!("Unsupported operation: {:?} in line {}", node.token.token_type, node.line);
        }
    }
}

pub fn get_index_from_register(register: &str) -> usize {
    match register {
        "R0" => 0,
        "R1" => 1,
        "R2" => 2,
        "R3" => 3,
        _ => panic!("Invalid register: {}", register),
    }
}

/// # Run Compiled Code
/// * Lexes Code
/// * Parses Code
/// * Compiles Code
/// * Runs Code
/// # Panics
/// This function will panic if the code is invalid
pub fn run_compiled_code_with_debugging(contents: &String) {
    println!("- CLANG:");
    let lex = get_lexer_lines(contents);
    let par = parse(lex);
    for l in par.clone() { println!("{}", fmt_expr(&l.unwrap())) }
    
    println!("\n- ASM:"); 
    let value = interpret(par.clone());
    println!("{value}");

    println!("\n- OUT:");
    println!("{}", compile_assembly_to_binary(&value));

    println!("\n- RUN:");
    let contents = compile_assembly_to_binary(&value);
    let bytes = string_to_bytes(contents.as_str());
    let mut computer: vc_8bit::Computer = vc_8bit::Computer::new();
    computer.ram.insert_bytes(bytes);
    computer.run();
}