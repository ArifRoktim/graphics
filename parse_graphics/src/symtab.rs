use super::{Axis, ParseError};
use lib_graphics::Shine;
use std::collections::HashMap;

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

type SymbolTable = HashMap<String, Symbol>;
#[derive(Debug)]
pub struct Operation {
    pub command: Command,
    pub light_const: Option<String>,
}
impl Operation {
    pub fn new(command: Command, light_const: Option<String>) -> Operation {
        Operation { command, light_const }
    }
}

#[derive(Debug)]
pub struct ToDoList {
    pub ops: Vec<Operation>,
    pub symbols: SymbolTable,
}
impl ToDoList {
    pub fn push_op(
        &mut self,
        command: Command,
        light_const: Option<String>,
    ) -> Result<(), ParseError> {
        let op = Operation::new(command, light_const);
        self.ops.push(op);
        Ok(())
    }

    pub fn add_sym(&mut self, k: String, v: Symbol) -> Result<(), ParseError> {
        self.symbols.insert(k, v);
        Ok(())
    }
}
impl Default for ToDoList {
    fn default() -> Self {
        let ops = vec![];
        let symbols: SymbolTable = HashMap::new();
        ToDoList { ops, symbols }
    }
}
