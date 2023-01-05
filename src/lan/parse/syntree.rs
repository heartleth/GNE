use std::collections::HashSet;
use super::super::STNForPy;
use crate::assembling::*;

#[derive(Clone, Debug)]
pub struct Morpheme {
    pub text: String,
    pub name: String
}

#[derive(Clone)]
pub struct SyntaxTree {
    pub children: Vec<SyntaxTreeNode>,
    pub name: String,
}

#[derive(Clone)]
pub enum SyntaxTreeNode {
    Category(SyntaxTree),
    Vocab(Morpheme)
}

impl SyntaxTreeNode {
    pub fn tostn(&self, strpass :&HashSet<String>) -> STNForPy {
        match self {
            Self::Vocab(v) => STNForPy {
                partname: self.get_name().clone(),
                strpass: strpass.clone(),
                vocab: Some(v.clone()),
                mode: false,
                category: None,

            },
            Self::Category(c) => STNForPy {
                partname: self.get_name().clone(),
                category: Some(c.clone()),
                strpass: strpass.clone(),
                mode: true,
                vocab: None
            }
        }
    }

    pub fn new_category(name :&str)->SyntaxTreeNode {
        return SyntaxTreeNode::Category(SyntaxTree {
            name: String::from(name),
            children: Vec::new()
        });
    }

    pub fn new_morpheme(name :String, text :String)->SyntaxTreeNode {
        return SyntaxTreeNode::Vocab(Morpheme {
            name: name,
            text: String::from(text)
        });
    }

    pub fn push_category(&mut self, node :SyntaxTreeNode)->Option<()> {
        if let SyntaxTreeNode::Category(c) = self {
            c.children.push(node);
            return Some(());
        }
        else {
            return None;
        }
    }

    pub fn collect(&self, delim :&str)->String {
        match self {
            Self::Category(c) => {
                let c :Vec<_> = c.children.iter().map(|e| e.collect(delim)).collect();
                c.join(delim)
            },
            Self::Vocab(v) => {
                String::from(&v.text)
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn collect_verbose(&self, delim :&str)->String {
        let stpl = vec![
            "String",
            "Integer",
            "Ident"
        ];
        match self {
            Self::Category(c) => {
                let (part, _) = &c.name[..].split_once('@').unwrap();
                if stpl.contains(part) {
                    format!("{}['{}']", &c.name, self.collect(""))
                }
                else {
                    let cc :Vec<_> = c.children.iter().map(|e| e.collect_verbose(delim)).collect();
                    format!("{}[ {} ]", &c.name, assemble(&cc.join(delim)))
                }
            },
            Self::Vocab(v) => {
                if v.text == " " {
                    return String::new();
                }
                format!("{}", &v.text.trim())
            }
        }
    }

    pub fn is_category(&self) -> bool {
        match self {
            Self::Category(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_vocab(&self) -> bool {
        match self {
            Self::Vocab(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn get_vocab(&self) -> Option<&Morpheme> {
        match self {
            Self::Vocab(v) => Some(v),
            _ => None
        }
    }
    
    #[allow(dead_code)]
    pub fn get_category(&self) -> Option<&SyntaxTree> {
        match self {
            Self::Category(v) => Some(v),
            _ => None
        }
    }
    
    pub fn get_name(&self) -> &String {
        match self {
            Self::Category(v) => &v.name,
            Self::Vocab(v) => &v.name
        }
    }
}