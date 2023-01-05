pub mod dictionary;
pub mod lanparser;
pub mod concrete;
pub mod parse;

pub use crate::assembling;
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
pub enum LanError<E> {
    NoDictionaryError,
    LanSyntaxError,
    ParsingError,
    NoMainError,
    Error(E),
    SimpleError
}

pub struct Parser {
    lcs :Vec<String>,
    dictionary :Option<Rc<dictionary::DictionaryRc>>
}

pub struct RefParser<'p> {
    dict: &'p dictionary::Dictionary<'p>,
    lan: &'p lanparser::LanRules<'p>
}

impl Parser {
    #[allow(dead_code)]
    pub fn new() -> Result<Parser, ()> {
        Ok(Parser {
            lcs: Vec::new(),
            dictionary: None
        })
    }
    
    pub fn open(lan_path :&str, dictionary_path :&str) -> Result<Parser, LanError<usize>> {
        let dict = dictionary::load_dictionaryrc(&std::fs::read_to_string(dictionary_path).ok().ok_or(LanError::NoDictionaryError)?[..]);
        Ok(Parser {
            lcs: vec![ std::fs::read_to_string(lan_path).unwrap() ],
            dictionary: Some(Rc::from(dict))
        })
    }
    
    #[allow(dead_code)]
    pub fn open_lan(lan_path :&str) -> Result<Parser, ()> {
        Ok(Parser {
            lcs: vec![ std::fs::read_to_string(lan_path).unwrap() ],
            dictionary: None
        })
    }

    // pub fn context<F, E:std::error::Error>(&self, block :F) -> Result<(), LanError<E>> where F:(Fn(lanparser::LanRules, dictionary::Dictionary) -> Result<(), E>) {
    pub fn context<F>(&self, block :F) -> pyo3::PyResult<()> where F:(Fn(RefParser) -> pyo3::PyResult<()>) {
        // let lan = lanparser::load_lan(&self.lcs).map_err(|_| LanError::LanSyntaxError)?;
        let lan = lanparser::load_lan(&self.lcs).unwrap();
        let dict = dictionary::dictrc_to_dict(&&self.dictionary.as_ref().unwrap());
        let rp = RefParser { dict: &dict, lan: &lan };
        block(rp)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn load_lan(&mut self, lan_path :&str) {
        self.lcs.push(std::fs::read_to_string(lan_path).unwrap());
    }

    #[allow(dead_code)]
    pub fn load_dict(&mut self, dictionary :&Rc<dictionary::DictionaryRc>) {
        self.dictionary = Some(dictionary.clone());
    }
}

pub struct ParseResult {
    pub tree :parse::syntree::SyntaxTreeNode,
    pub length :usize
}

impl<'p> RefParser<'p> {
    pub fn reparse_raw(&self, v: &Vec<char>) -> Result<Option<(parse::syntree::SyntaxTreeNode, usize)>, LanError<()>> {
        Ok(parse::parse(v, self.lan.get("main").ok_or(LanError::NoMainError)?.build(Vec::new()), self.lan, self.dict))
    }

    pub fn parse_raw(&self, v: &Vec<char>) -> Result<Option<(parse::syntree::SyntaxTreeNode, usize)>, LanError<()>> {
        parse::init_parse();
        self.reparse_raw(v)
    }

    pub fn parse<T>(&self, text: T) -> Result<ParseResult, LanError<()>> where String: From<T> {
        let v = assembling::disassemble(&String::from(text)[..]);
        self.parse_raw(&v)?.ok_or(LanError::ParsingError).map(|e| ParseResult {
            length :e.1,
            tree :e.0
        })
    }
    
    #[allow(dead_code)]
    pub fn parse_check<T>(&self, text: T) -> Result<usize, LanError<()>> where String: From<T> {
        let v = assembling::disassemble(&String::from(text)[..]);
        parse::init_parse();
        Ok(self.reparse_raw(&v)?.ok_or(LanError::ParsingError)?.1)
    }
}

// use std::collections::HashMap;
use std::collections::HashSet;
use parse::syntree::*;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::IntoPy;

impl SyntaxTreeNode {
    pub fn run_py(&self, module :&PyModule, strpass :&HashSet<String>) -> pyo3::PyResult<pyo3::PyObject> {
        match self {
            Self::Vocab(v) => Ok(v.text.clone().into_py(module.py())),
            Self::Category(c) => {
                let (category, rule) = c.name.split_once('@').unwrap();

                if strpass.contains(&c.name) {
                    Ok(self.collect("").into_py(module.py()))
                }
                else if module.getattr(category).is_err() || module.getattr(category)?.getattr(rule).is_err() {
                    for child in &c.children {
                        if child.is_category() {
                            child.run_py(module, strpass)?;
                        }
                    }
                    Ok(0.into_py(module.py()))
                }
                else{
                    // let mut nthname = HashMap::new();
                    // let mut argm = HashMap::new();
                    let mut argv = Vec::new();
                    
                    for child in &c.children {
                        argv.push(child.tostn(strpass).into_py(module.py()));
                        // if let SyntaxTreeNode::Category(c) = child {
                        //     let childname = c.name.split_once('@').unwrap().0;
                        //     if let Some(n) = nthname.get_mut(childname) {
                        //         *n = *n + 1;
                        //         argm.insert(format!("{}_{}", childname, n), child.tostn(strpass).into_py(module.py()));
                        //     }
                        //     else {
                        //         nthname.insert(&childname[..], 0);
                        //         argm.insert(childname.to_string(), child.tostn(strpass).into_py(module.py()));
                        //     }
                        // }
                    }
                    Ok(module.getattr(category)?.call_method1(rule, PyTuple::new(module.py(), argv))?.into_py(module.py()))
                    // Ok(module.getattr(category)?.call_method(rule, PyTuple::new(module.py(), argv), Some(argm.into_py_dict(module.py())))?.into_py(module.py()))
                }
            }
        }
    }
}

#[derive(Clone)]
#[pyo3::pyclass]
pub struct STNForPy {
    category :Option<SyntaxTree>,
    strpass :HashSet<String>,
    vocab :Option<Morpheme>,
    partname :String,
    mode :bool
}

#[pyo3::pymethods]
impl STNForPy {
    pub fn __call__(&self) -> PyResult<PyObject> {
        let h = std::fs::read_to_string("lang.py").unwrap();
        Python::with_gil(|py| {
            let m = PyModule::from_code(py, &h, "hello", "hello")?;
            if self.mode {
                SyntaxTreeNode::Category(self.category.as_ref().unwrap().clone()).run_py(m, &self.strpass)
            }
            else {
                Ok(self.vocab.as_ref().unwrap().text.clone().into_py(py))
            }
        })
    }

    pub fn code(&self) -> PyResult<String> {
        if self.mode {
            let c :Vec<String> = self.category.as_ref().unwrap().children.iter().map(|e| e.collect("")).collect();
            Ok(c.join(""))
        }
        else {
            Ok(self.vocab.as_ref().unwrap().text.clone())
        }
    }

    pub fn name(&self) -> PyResult<String> {
        Ok(self.partname.clone())
    }
}