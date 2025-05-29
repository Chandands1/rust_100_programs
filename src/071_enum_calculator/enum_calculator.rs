#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Power(f64, f64),
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add(a, b) => a + b,
            Operation::Subtract(a, b) => a - b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => {
                if *b == 0.0 {
                    panic!("Division by zero!");
                }
                a / b
            },
            Operation::Power(a, b) => a.powf(*b),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Operation::Add(a, b) => format!("{} + {}", a, b),
            Operation::Subtract(a, b) => format!("{} - {}", a, b),
            Operation::Multiply(a, b) => format!("{} × {}", a, b),
            Operation::Divide(a, b) => format!("{} ÷ {}", a, b),
            Operation::Power(a, b) => format!("{} ^ {}", a, b),
        }
    }
}

fn main() {
    let operations = [
        Operation::Add(5.0, 3.0),
        Operation::Subtract(10.0, 7.0),
        Operation::Multiply(4.0, 2.5),
        Operation::Divide(10.0, 2.0),
        Operation::Power(2.0, 8.0),
    ];

    for op in operations {
        println!(
            "{} = {:.2}",
            op.to_string(),
            op.calculate()
        );
    }

    // Example of error case
    // let div_zero = Operation::Divide(5.0, 0.0);
    // println!("{} = {:.2}", div_zero.to_string(), div_zero.calculate());
}