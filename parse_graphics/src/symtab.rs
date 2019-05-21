use lib_graphics::Shine;
use std::collections::HashMap;
use super::{AstNode, Command};

#[derive(Debug)]
pub enum Symbol {
    Constant(Shine, Shine, Shine),
    //Knob,
    //Light, etc..
}

type SymbolTable<'a> = HashMap<&'a str, Symbol>;
#[derive(Debug)]
pub struct Operation<'a> {
    pub command: Command,
    // TODO: Instead, borrow a slice of `AstNode`s
    pub args: Vec<AstNode>,
    pub sym_name: Option<&'a str>,
}
impl<'a> Operation<'a> {
    pub fn new(command: Command, args: Vec<AstNode>, sym_name: Option<&str>) -> Operation {
        Operation { command, args, sym_name }
    }
}

#[derive(Debug)]
pub struct ToDoList<'a> {
    pub ops: Vec<Operation<'a>>,
    pub symbols: SymbolTable<'a>,
}
impl<'a> ToDoList<'a> {
    pub fn push_op(
        &mut self,
        command: &Command,
        args: Vec<AstNode>,
        sym_name: Option<&'a str>,
    ) {
        let op = Operation::new(command.to_owned(), args, sym_name);
        self.ops.push(op);
    }

    pub fn add_sym(&mut self, k: &'a str, v: Symbol) {
        self.symbols.insert(k, v);
    }
}
impl<'a> Default for ToDoList<'a> {
    fn default() -> Self {
        let ops = vec![];
        let symbols: SymbolTable = HashMap::new();
        ToDoList { ops, symbols }
    }
}
