#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn new(descriptor: &str) -> Self {
        match descriptor {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Operand {
    OldValue,
    Number(u32),
}

impl Operand {
    fn new(descriptor: &str) -> Self {
        match descriptor {
            "old" => Self::OldValue,
            number => Self::Number(number.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    operand_a: Operand,
    operand_b: Operand,
    operator: Operator,
}

impl Operation {
    pub fn new(descriptor: &str) -> Self {
        let mut descriptor = descriptor.split(" ");
        let operand_a = descriptor.next().map(Operand::new).unwrap();
        let operator = descriptor.next().map(Operator::new).unwrap();
        let operand_b = descriptor.next().map(Operand::new).unwrap();

        Self {
            operand_a,
            operand_b,
            operator,
        }
    }

    pub fn execute(&self, old_value: u32) -> u32 {
        let operand_a = if let Operand::Number(number) = self.operand_a {
            number
        } else {
            old_value
        };
        let operand_b = if let Operand::Number(number) = self.operand_b {
            number
        } else {
            old_value
        };

        match self.operator {
            Operator::Add => operand_a + operand_b,
            Operator::Multiply => operand_a * operand_b,
        }
    }
}
