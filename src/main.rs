pub mod assembling;
mod lan;

use std::collections::HashSet;
use std::fs::read_to_string;
use pyo3::prelude::*;
use pyo3::types::*;
use lan::Parser;

use std::time::Instant;

fn main() -> PyResult<()> {
    let h = read_to_string("lang.py").unwrap();
    
    Parser::open("language.lan", "dictionary.dic").unwrap().context(|lan| {
        let now = Instant::now();
        let parsed = lan.parse(read_to_string("code.ㄹ").unwrap().trim()).unwrap().tree;
        println!("{:?}", now.elapsed());
        // println!("{}", parsed.collect_verbose(" "));
        Python::with_gil(|py| {
            let m = PyModule::from_code(py, &h, "hello", "hello")?;
            parsed.run_py(&m, &HashSet::from_iter(m.getattr("strpass")?.cast_as::<PySet>()?.iter().map(|e| e.to_string())))?;
            Ok(())
        })
    })
}