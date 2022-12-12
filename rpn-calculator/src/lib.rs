#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut calc_stack = Vec::new();
    for input in inputs {
        match &input {
            CalculatorInput::Value(val) => calc_stack.push(*val),
            _ => {
                let (rhs, lhs) = (calc_stack.pop(), calc_stack.pop());
                match (lhs, rhs) {
                    (Some(lhs), Some(rhs)) => {
                        let result = match &input {
                            CalculatorInput::Add => lhs + rhs,
                            CalculatorInput::Subtract => lhs - rhs,
                            CalculatorInput::Divide => lhs / rhs,
                            CalculatorInput::Multiply => lhs * rhs,
                            _ => unreachable!()
                        };
                        calc_stack.push(result);
                    }
                    _ => return None
                }

            }
        }
    }
    match calc_stack.len() {
        1 => calc_stack.pop(),
        _ => None
    } 
}
