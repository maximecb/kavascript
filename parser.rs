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

pub type Result<T> = std::result::Result<T, ParseError>;

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

    // Read a character from the input
    pub fn read_ch(&mut self) -> Result<char>
    {
        let ch = self.peek_ch();

        // Strictly reject characters outside of the useful ASCII range
        // We can remove this restriction later if needed
        if (ch < '\x20' || ch > '\x7E') && (ch != '\n' && ch != '\t' && ch != '\r')
        {
            //char hexStr[64];
            //sprintf(hexStr, "0x%02X", (int)ch);

            // FIXME:
            /*
            throw ParseError(
                *this,
                "invalid character in input, " + std::string(hexStr)
            );
            */

            return Result::Err(ParseError::new(self, "invalid character in input"));
        }

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

        return Result::Ok(ch);
    }

    // Match a string in the input, no preceding whitespace allowed
    pub fn match_exact(&mut self, token: &str) -> Result<bool>
    {
        // NOTE: we need to take care of the position, line number, etc.
        // May want to use peek/read_ch for that.




        return Result::Ok(false);
    }

    // Match a string in the input, ignoring preceding whitespace
    pub fn match_token(&mut self, token: &str) -> Result<bool>
    {
        // Consume preceding whitespace
        self.eat_ws()?;




        
        return Result::Ok(false);
    }

    // Consume whitespace and comments
    pub fn eat_ws(&mut self) -> Result<()>
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
                self.read_ch()?;
                continue;
            }

            // If this is a single-line comment
            if self.match_exact("//")?
            {
                // Read until and end of line is reached
                loop
                {
                    if self.eof()
                    {
                        break;
                    }

                    if self.read_ch()? == '\n'
                    {
                        break;
                    }
                }

                continue;
            }

            /*
            // If this is a multi-line comment
            if (match("/*"))
            {
                // Read until the end of the comment
                for (;;)
                {
                    if (eof())
                    {
                        throw ParseError(
                            *this,
                            "end of input in multiline comment"
                        );
                    }

                    if (readCh() == '*' && match("/"))
                    {
                        break;
                    }
                }

                continue;
            }
            */*/

            // This isn't whitespace, stop
            break;
        }

        return Result::Ok(());
    }

    // TODO: expect
    // Skip the expect_exact version for now. YAGNI.







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
