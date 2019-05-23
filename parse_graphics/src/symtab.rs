use lib_graphics::Shine;
use std::collections::HashMap;
use super::{Axis, ParseError};

#[derive(Debug)]
pub enum Symbol {
    Constant(Shine, Shine, Shine),
    //Knob,
    //Light, etc..
}

#[derive(Clone, Debug)]
pub struct NOOP;

#[derive(Clone, Debug)]
pub enum Command {
    Push(),
    Pop(),
    Display(),
    Save(String),
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    Rotate(Axis, f64),
    Cuboid(f64, f64, f64, f64, f64, f64),
    Sphere(f64, f64, f64, f64),
    Torus(f64, f64, f64, f64, f64),
    Line(f64, f64, f64, f64, f64, f64),
    Constants(NOOP),
}

type SymbolTable<'a> = HashMap<&'a str, Symbol>;
#[derive(Debug)]
pub struct Operation<'a> {
    pub command: Command,
    pub light_const: Option<&'a str>,
}
impl<'a> Operation<'a> {
    pub fn new(command: Command, light_const: Option<&str>) -> Operation {
        Operation { command, light_const }
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
        command: Command,
        light_const: Option<&'a str>,
    ) -> Result<(), ParseError>
    {
        let op = Operation::new(command, light_const);
        self.ops.push(op);
        Ok(())
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
