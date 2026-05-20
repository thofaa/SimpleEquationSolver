mod pyo3temp;

use pyo3temp::{solve_equation_with_python, EquationJob};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let equation = args
        .first()
        .cloned()
        .unwrap_or_else(|| "2*x + 3 = 7".to_string());
    let variable = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "x".to_string());

    let job = EquationJob::new(equation, variable);

    match solve_equation_with_python(&job) {
        Ok(answer) => {
            println!("solutions: {:?}", answer.solutions);
        }
        Err(error) => {
            eprintln!("bridge error: {error}");
            std::process::exit(1);
        }
    }
}
