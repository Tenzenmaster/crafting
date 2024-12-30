use crate::*;

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn run(mut self, chunk: Chunk) -> Result<Option<Value>, &'static str> {
        for code in chunk.code {
            use Opcode::*;
            eprintln!("{:?}", self.stack);
            match code {
                Return => return Ok(self.stack.pop()),
                Constant(index) => self.stack.push(chunk.constants[index].clone()),
                Negate => {
                    match self.stack.last_mut().unwrap() {
                        Value::Nil => return Err("Cannot negate nil"),
                        Value::Bool(_) => return Err("Cannot negate bool"),
                        Value::Number(n) => *n *= -1.0,
                    }
                },
                Add => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a + b));
                },
                Subtract => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a - b));
                },
                Multiply => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a * b));
                },
                Divide => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Number(a / b));
                },
            }
        }

        Err("No return statement in chunk")
    }

    fn pop_number(&mut self) -> Result<f64, &'static str> {
        let Some(value) = self.stack.pop() else {
            return Err("pop_number was None");
        };
        let Value::Number(n) = value else {
            return Err("pop_number received non number");
        };
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm1() {
        let mut chunk = Chunk::new();
        chunk.write_constant(Value::Number(50.0), 1);
        chunk.write_constant(Value::Number(-1.0), 1);
        chunk.write_constant(Value::Number(13.9), 1);
        chunk.write_code(Opcode::Negate, 1);
        chunk.write_code(Opcode::Subtract, 1);
        chunk.write_code(Opcode::Divide, 1);
        chunk.write_code(Opcode::Return, 2);

        let vm = VM::new();
        assert_eq!(vm.run(chunk), Ok(Some(Value::Number(50.0 / (-1.0 + 13.9)))));
    }
}
