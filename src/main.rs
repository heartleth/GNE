pub mod assembling;
mod lan;

use pyo3::exceptions::PyTypeError;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;
use pyo3::prelude::*;
use pyo3::types::*;
use lan::Parser;
use std::env;

fn main() -> PyResult<()> {
    let h = read_to_string("lang.py").unwrap();
    let args :Vec<String> = env::args().collect();
    let c = "example.yhj".to_string();
    let filename = args.get(1).unwrap_or(&c);
    
    Parser::open("language.lan", "dictionary.dic").unwrap().context(|lan| {
        let filename = filename.clone();
        
        let now = Instant::now();
        if let Ok(p) = lan.parse(read_to_string(filename).unwrap().trim()) {
            let parsed = p.tree;
            println!("{:?}", now.elapsed());
            if args.contains(&"-ast".to_string()) {
                println!("{}", parsed.collect_verbose(" "));
            }
            Python::with_gil(|py| {
                let m = PyModule::from_code(py, &h, "hello", "hello")?;
                parsed.run_py(&m, &HashSet::from_iter(m.getattr("strpass")?.cast_as::<PySet>()?.iter().map(|e| e.to_string())))?;
                Ok(())
            })
        }
        else {
            Err(PyErr::new::<PyTypeError, _>("Parsing error"))
        }
    })
}