use std::fs::File;
use std::io;
use std::io::Read;

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
    pub fn new(input_str: String, src_name: String) -> Self
    {
        Input {
            input_str: input_str.chars().collect(),
            src_name: src_name,
            pos: 0,
            line_no: 1,
            col_no: 1
        }
    }

    // Test if the end of the input has been reached
    pub fn eof(&self) -> bool
    {
        return self.pos >= self.input_str.len();
    }

    // Peek at a character from the input
    pub fn peek_ch(&self) -> char
    {
        if self.pos > self.input_str.len()
        {
            return '\0';
        }

        return self.input_str[self.pos];
    }

    // Consume a character from the input
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

    // Consume whitespace
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

            let ch = self.peek_ch();

            // Consume whitespace characters
            if ch == ' ' || ch == '\t'
            {
                self.eat_ch();
                continue;
            }

            // This isn't whitespace, stop
            break;
        }
    }

    // Match a string in the input, no preceding whitespace allowed
    pub fn match_exact(&mut self, token: &str) -> bool
    {
        if token.len() < self.input_str.len() {
            return false;
        }

        let token_chars: Vec<char> = token.chars().collect();
        let end_pos = self.pos + token_chars.len();

        if token_chars == self.input_str[self.pos..end_pos] {
            for i in 0..token_chars.len() {
                self.eat_ch();
            }

            return true;
        }

        return false;
    }

    // Match a string in the input, ignoring preceding whitespace
    pub fn match_token(&mut self, token: &str) -> bool
    {
        // Consume preceding whitespace
        self.eat_ws();

        return self.match_exact(token);
    }




    pub fn parse_error()
    {



    }


    // TODO: this can produce a parse error
    pub fn expect_token(&mut self, token: &str)
    {

    }



    // TODO: this can also produce a parse error if there is no input
    pub fn parse_int(&mut self) -> i64
    {



        return 0;
    }





    // TODO: expect
    // Skip the expect_exact version for now. YAGNI.







}





fn parse_atom()
{

}









// TODO:
// parse_file

// TODO:
// assert_parses!()
// assert_fails!()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {


        //assert_eq!(2 + 2, 4);




    }
}
