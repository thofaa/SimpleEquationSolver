use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationJob {
    pub equation: String,
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationAnswer {
    pub solutions: Vec<String>,
}

impl EquationJob {
    pub fn new(equation: String, variable: String) -> Self {
        Self { equation, variable }
    }
}

pub fn solve_equation_with_python(job: &EquationJob) -> io::Result<EquationAnswer> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input_path = manifest_dir.join("equation.json");
    let output_path = manifest_dir.join("answer.json");
    let script_path = manifest_dir.join("python").join("solver.py");

    write_json_file(&input_path, job)?;
    run_python_solver(&script_path, &input_path, &output_path)?;

    let output_text = fs::read_to_string(&output_path)?;
    let answer: EquationAnswer = serde_json::from_str(&output_text)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    Ok(answer)
}

fn write_json_file(path: &Path, job: &EquationJob) -> io::Result<()> {
    let json = serde_json::to_string_pretty(job)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(path, json)
}

fn run_python_solver(script_path: &Path, input_path: &Path, output_path: &Path) -> io::Result<()> {
    let try_python = |program: &str, extra_args: &[&str]| -> io::Result<bool> {
        let status = Command::new(program)
            .args(extra_args)
            .arg(script_path)
            .arg(input_path)
            .arg(output_path)
            .status();

        match status {
            Ok(status) if status.success() => Ok(true),
            Ok(status) => Err(io::Error::other(format!(
                "python process exited with {status}"
            ))),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    };

    if try_python("python", &[])? {
        return Ok(());
    }

    if try_python("py", &["-3"])? {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "could not find a usable python executable",
    ))
}