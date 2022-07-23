use std::fs::File;
use std::io;
use std::io::Read;
use std::fmt;
use crate::vm::*;

#[derive(Debug)]
pub struct ParseError
{
    msg: String,
    line_no: u32,
    col_no: u32,
}

impl ParseError
{
    pub fn new(input: &Input, msg: &str) -> Self
    {
        ParseError {
            msg: msg.to_string(),
            line_no: input.line_no,
            col_no: input.col_no
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error")
    }
}

pub struct Input
{
    // Input string to be parsed
    input_str: Vec<char>,

    // Input source name
    src_name: String,

    // Current position in the input string
    pos: usize,

    // Current line number
    line_no: u32,

    // Current column number
    col_no : u32,
}

impl Input
{
    pub fn new(input_str: &str, src_name: &str) -> Self
    {
        Input {
            input_str: input_str.chars().collect(),
            src_name: src_name.to_string(),
            pos: 0,
            line_no: 1,
            col_no: 1
        }
    }

    /// Test if the end of the input has been reached
    pub fn eof(&self) -> bool
    {
        return self.pos >= self.input_str.len();
    }

    /// Peek at a character from the input
    pub fn peek_ch(&self) -> char
    {
        if self.pos > self.input_str.len()
        {
            return '\0';
        }

        return self.input_str[self.pos];
    }

    /// Consume a character from the input
    pub fn eat_ch(&mut self) -> char
    {
        let ch = self.peek_ch();

        // Move to the next char
        self.pos += 1;

        if ch == '\n'
        {
            self.line_no += 1;
            self.col_no = 1;
        }
        else
        {
            self.col_no += 1;
        }

        return ch;
    }

    /// Consume whitespace
    pub fn eat_ws(&mut self)
    {
        // Until the end of the whitespace
        loop
        {
            // If we are at the end of the input, stop
            if self.eof()
            {
                break;
            }

            // Single-line comments
            if self.match_exact("//")
            {
                loop
                {
                    // If we are at the end of the input, stop
                    if self.eof() || self.eat_ch() == '\n'
                    {
                        break;
                    }
                }
            }

            let ch = self.peek_ch();

            // Consume whitespace characters
            if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
            {
                self.eat_ch();
                continue;
            }

            // This isn't whitespace, stop
            break;
        }
    }

    /// Match a string in the input, no preceding whitespace allowed
    pub fn match_exact(&mut self, token: &str) -> bool
    {
        let token_chars: Vec<char> = token.chars().collect();
        let end_pos = self.pos + token_chars.len();

        if end_pos > self.input_str.len() {
            return false;
        }

        if token_chars == self.input_str[self.pos..end_pos] {
            for i in 0..token_chars.len() {
                self.eat_ch();
            }

            return true;
        }

        return false;
    }

    /// Match a string in the input, ignoring preceding whitespace
    pub fn match_token(&mut self, token: &str) -> bool
    {
        // Consume preceding whitespace
        self.eat_ws();

        return self.match_exact(token);
    }

    /// Shortcut for yielding a parse error wrapped in a result type
    pub fn parse_error<T>(&self, msg: &str) -> Result<T, ParseError>
    {
        Err(ParseError::new(self, msg))
    }

    /// Produce an error if the input doesn't match a given token
    pub fn expect_token(&mut self, token: &str) -> Result<(), ParseError>
    {
        if self.match_token(token) {
            return Ok(())
        }

        self.parse_error(&format!("expected token \"{}\"", token))
    }

    /// Parse a decimal integer value
    pub fn parse_int(&mut self) -> i64
    {
        let mut int_val = 0_i64;

        loop
        {
            if self.eof() {
                break;
            }

            let ch = self.peek_ch();
            let digit = ch.to_digit(10);

            if digit.is_none() {
                break
            }

            int_val = 10 * int_val + digit.unwrap() as i64;
            self.eat_ch();
        }

        return int_val;
    }

    /// Parse a string literal
    pub fn parse_str(&mut self) -> Result<String, ParseError>
    {
        let mut out = String::new();

        loop
        {
            if self.eof() {
                return self.parse_error("unexpected end of input while parsing integer");
            }

            let ch = self.eat_ch();

            if ch == '\"' {
                break
            }

            out.push(ch);
        }

        return Ok(out);
    }
}

/// Parse an atomic expression
fn parse_atom(input: &mut Input, fun: &mut Function) -> Result<(), ParseError>
{
    let ch = input.peek_ch();

    // Decimal integer literal
    if ch.is_digit(10) {
        let int_val = input.parse_int();
        fun.insns.push(Insn::Push { val: Value::Int(int_val) });
        return Ok(());
    }

    // Parenthesized expression
    if ch == '(' {
        parse_expr(input, fun)?;
        input.expect_token(")")?;
        return Ok(());
    }

    // Variable reference
    if ch == '_' || ch.is_alphanumeric() {
        todo!()
    }

    input.parse_error("unknown atomic expression")
}

// TODO
// Operators and precedence
//
//

/// Parse an expression
fn parse_expr(input: &mut Input, fun: &mut Function) -> Result<(), ParseError>
{


    parse_atom(input, fun)?;










    Ok(())
}

/// Parse a statement
fn parse_stmt(input: &mut Input, fun: &mut Function) -> Result<(), ParseError>
{






    // Try to parse this as an expression statement
    parse_expr(input, fun)?;
    fun.insns.push(Insn::Pop);
    input.expect_token(";")
}

/// Parse a function definition
fn parse_fun(input: &mut Input) -> Result<Function, ParseError>
{
    todo!();
}

/// Parse a single unit of source code (e.g. one source file)
fn parse_unit(input: &mut Input) -> Result<Function, ParseError>
{
    let mut unit_fun = Function::new(&input.src_name);

    loop
    {
        if input.eof() {
            break;
        }

        parse_stmt(input, &mut unit_fun)?;

        // TODO: detect function keyword
    }

    Ok(unit_fun)
}

// TODO:
// parse_file

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn int_token_int()
    {
        let mut input = Input::new("1 + 2", "input");
        input.eat_ws();
        assert_eq!(input.parse_int(), 1);
        assert!(input.match_token("+"));
        input.eat_ws();
        assert_eq!(input.parse_int(), 2);
        assert!(input.eof());
    }

    #[test]
    fn simple_str()
    {
        let mut input = Input::new(" \"foobar\"", "input");
        input.eat_ws();
        assert!(input.match_token("\""));
        assert_eq!(input.parse_str().unwrap(), "foobar");
        input.eat_ws();
        assert!(input.eof());
    }

    #[test]
    fn single_line_comment()
    {
        let mut input = Input::new("1 // test\n  2", "input");
        assert_eq!(input.parse_int(), 1);
        input.eat_ws();
        dbg!(input.pos);
        assert_eq!(input.parse_int(), 2);
        assert!(input.eof());
    }





    #[test]
    fn simple_unit()
    {
        let mut input = Input::new("1;", "src");
        parse_unit(&mut input).unwrap();



    }






}
