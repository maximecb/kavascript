use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
use std::fmt;
use std::cmp::max;
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

#[derive(Debug, Clone)]
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
        if self.pos >= self.input_str.len()
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
            if ch.is_ascii_whitespace()
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

    /// Parse a C-style alphanumeric identifier
    pub fn parse_ident(&mut self) -> String
    {
        let mut ident = String::new();

        loop
        {
            if self.eof() {
                break;
            }

            let ch = self.peek_ch();

            if ch != '_' && !ch.is_ascii_alphanumeric() {
                break;
            }

            ident.push(ch);
            self.eat_ch();
        }

        return ident;
    }
}

struct Scope
{
    /// Map of variables to local indices
    vars: HashMap<String, usize>,

    /// Function this scope resides in
    fun: *mut Function,

    /// Parent scope
    parent: Option<*mut Scope>,

    /// Next local idx to assign
    next_idx: usize,
}

impl Scope
{
    fn new(fun: &mut Function) -> Scope
    {
        Scope {
            vars: HashMap::default(),
            fun: fun as *mut Function,
            parent: None,
            next_idx: 0,
        }
    }

    /// Create a new nested scope
    fn new_nested(parent: &mut Scope) -> Scope
    {
        Scope {
            vars: HashMap::default(),
            fun: parent.fun,
            parent: Some(parent as *mut Scope),
            next_idx: parent.next_idx,
        }
    }

    /// Declare a new variable
    fn decl_var(&mut self, ident: &str) -> Option<usize>
    {
        // Can't declare a variable twice in the same scope
        if let Some(local_idx) = self.vars.get(ident) {
            return None;
        }

        let local_idx = self.next_idx;
        self.next_idx += 1;

        self.vars.insert(ident.to_string(), local_idx);

        let mut fun = unsafe { &mut *self.fun };
        fun.num_locals = max(fun.num_locals, local_idx + 1);

        return Some(local_idx);
    }

    /// Look up a variable by name
    fn lookup(&self, ident: &str) -> Option<usize>
    {
        if let Some(idx) = self.vars.get(ident) {
            return Some(*idx);
        }
        else
        {
            if let Some(parent_ptr) = self.parent {
                let parent = unsafe { &*parent_ptr };
                return parent.lookup(ident);
            }
            else
            {
                return None;
            }
        }
    }
}

/// Parse an atomic expression
fn parse_atom(input: &mut Input, fun: &mut Function, scope: &mut Scope) -> Result<(), ParseError>
{
    input.eat_ws();
    let ch = input.peek_ch();

    // Decimal integer literal
    if ch.is_digit(10) {
        let int_val = input.parse_int();
        fun.insns.push(Insn::Push { val: Value::Int64(int_val) });
        return Ok(());
    }

    // Parenthesized expression
    if ch == '(' {
        input.eat_ch();
        parse_expr(input, fun, scope)?;
        input.expect_token(")")?;
        return Ok(());
    }

    // Variable reference
    if ch == '_' || ch.is_ascii_alphanumeric() {
        let ident = input.parse_ident();

        let local_idx = scope.lookup(&ident);

        if local_idx.is_none() {
            return input.parse_error(&format!("undeclared variable {}", ident));
        }

        // If this is actually an assignment
        if input.match_token("=") {
            // Parse the expression to assign
            parse_expr(input, fun, scope)?;

            fun.insns.push(Insn::Dup);
            fun.insns.push(Insn::SetLocal{ idx: local_idx.unwrap() });
        }
        else
        {
            fun.insns.push(Insn::GetLocal{ idx: local_idx.unwrap() });
        }

        return Ok(());
    }

    input.parse_error("unknown atomic expression")
}

struct OpInfo
{
    op: &'static str,
    prec: usize,

    // TODO: assoc?
}

/// Binary operators and their precedence level
/// https://en.cppreference.com/w/c/language/operator_precedence
const BIN_OPS: [OpInfo; 5] = [
    OpInfo { op: "*", prec: 2 },
    OpInfo { op: "+", prec: 1 },
    OpInfo { op: "-", prec: 1 },
    OpInfo { op: "==", prec: 0 },
    OpInfo { op: "!=", prec: 0 },
];

/// Try to match a binary operator in the input
fn match_bin_op(input: &mut Input) -> Option<OpInfo>
{
    for op_info in BIN_OPS {
        if input.match_token(op_info.op) {
            return Some(op_info);
        }
    }

    None
}

fn emit_op(op: &str, fun: &mut Function)
{
    match op {
        "*" => fun.insns.push(Insn::Mul),
        "+" => fun.insns.push(Insn::Add),
        "-" => fun.insns.push(Insn::Sub),
        "==" => fun.insns.push(Insn::Eq),
        "!=" => fun.insns.push(Insn::Ne),
        _ => panic!()
    }
}

/// Parse a complex expression
/// This uses the shunting yard algorithm to parse infix expressions:
/// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
fn parse_expr(input: &mut Input, fun: &mut Function, scope: &mut Scope) -> Result<(), ParseError>
{
    // Operator stack
    let mut op_stack: Vec<OpInfo> = Vec::default();

    // Parse the first atomic expression
    parse_atom(input, fun, scope)?;

    loop
    {
        if input.eof() {
            break;
        }

        let new_op = match_bin_op(input);

        // If no operator could be matched, stop
        if new_op.is_none() {
            break
        }

        let new_op = new_op.unwrap();

        //println!("{}", new_op.op);

        while op_stack.len() > 0 {
            // Get the operator at the top of the stack
            let top_op = &op_stack[op_stack.len() - 1];

            if top_op.prec > new_op.prec {
                println!("emit {}", top_op.op);

                emit_op(top_op.op, fun);
                op_stack.pop();
            }
            else {
                break;
            }
        }

        op_stack.push(new_op);

        // There must be another expression following
        parse_atom(input, fun, scope)?;
    }

    // Emit all operators remaining on the operator stack
    while op_stack.len() > 0 {
        let top_op = &op_stack[op_stack.len() - 1];
        emit_op(top_op.op, fun);
        op_stack.pop();
    }

    Ok(())
}

/// Parse a statement
fn parse_stmt(input: &mut Input, fun: &mut Function, scope: &mut Scope) -> Result<(), ParseError>
{
    input.eat_ws();

    if input.match_token("return") {
        parse_expr(input, fun, scope)?;
        fun.insns.push(Insn::Return);
        input.expect_token(";")?;
        return Ok(());
    }

    // Variable declaration
    if input.match_token("let") {
        input.eat_ws();
        let ident = input.parse_ident();
        input.expect_token("=")?;
        parse_expr(input, fun, scope)?;
        input.expect_token(";")?;

        if let Some(local_idx) = scope.decl_var(&ident) {
            fun.insns.push(Insn::SetLocal{ idx: local_idx });
            return Ok(());
        }
        else
        {
            return input.parse_error(&format!("variable {} already declared", ident));
        }
    }

    // Block statement
    if input.match_token("{") {

        // Create a nested scope for the block
        let mut scope = Scope::new_nested(scope);

        loop
        {
            input.eat_ws();

            if input.eof() {
                return input.parse_error("unexpected end of input in block statement");
            }

            if input.match_token("}") {
                break;
            }

            parse_stmt(input, fun, &mut scope)?;
        }

        return Ok(());
    }

    // Assert statement
    if input.match_token("assert") {
        parse_expr(input, fun, scope)?;
        input.expect_token(";")?;

        // If the expression is true, don't panic
        fun.insns.push(Insn::IfTrue { offset: 1 });
        fun.insns.push(Insn::Panic);

        return Ok(());
    }

    // Try to parse this as an expression statement
    parse_expr(input, fun, scope)?;
    fun.insns.push(Insn::Pop);
    input.expect_token(";")
}

/// Parse a function definition
fn parse_fun(input: &mut Input) -> Result<Function, ParseError>
{
    todo!();
}

/// Parse a single unit of source code (e.g. one source file)
pub fn parse_unit(input: &mut Input) -> Result<Function, ParseError>
{
    let mut unit_fun = Function::new(&input.src_name);
    let mut scope = Scope::new(&mut unit_fun);

    loop
    {
        input.eat_ws();

        if input.eof() {
            break;
        }

        parse_stmt(input, &mut unit_fun, &mut scope)?;

        // TODO: detect function keyword
    }

    // Return nil
    unit_fun.insns.push(Insn::Push { val: Value::Nil });
    unit_fun.insns.push(Insn::Return);

    //dbg!(unit_fun.num_locals);
    //dbg!(&unit_fun.insns);

    Ok(unit_fun)
}

pub fn parse_file(file_name: &str) -> Function
{
    let data = fs::read_to_string(file_name)
        .expect(&format!("could not read input file {}", file_name));

    let mut input = Input::new(&data, file_name);

    parse_unit(&mut input).unwrap()
}

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
        parse_unit(&mut Input::new("", "src")).unwrap();
        parse_unit(&mut Input::new(" ", "src")).unwrap();
        parse_unit(&mut Input::new("1;", "src")).unwrap();
        parse_unit(&mut Input::new("1; ", "src")).unwrap();
    }

    #[test]
    fn infix_exprs()
    {
        // Should parse
        parse_unit(&mut Input::new("1 + 2;", "src")).unwrap();
        parse_unit(&mut Input::new("1 + 2 * 3;", "src")).unwrap();
        parse_unit(&mut Input::new("1 + 2 + 3;", "src")).unwrap();
        parse_unit(&mut Input::new("1 + 2 + 3 + 4;", "src")).unwrap();
        parse_unit(&mut Input::new("(1) + 2 + 3 * 4;", "src")).unwrap();

        // Should not parse
        assert!(parse_unit(&mut Input::new("1 + 2 +;", "src")).is_err());
    }

    #[test]
    fn stmts()
    {
        parse_unit(&mut Input::new("let x = 3;", "src")).unwrap();
        parse_unit(&mut Input::new("let x = 3; let y = 5;", "src")).unwrap();
        parse_unit(&mut Input::new("{ let x = 3; x; } let y = 4;", "src")).unwrap();
    }
}
