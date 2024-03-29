pub mod fit_rules;
pub mod syntree;

use syntree::*;

use super::concrete::ConcretePart;
use super::lanparser::*;

use super::dictionary::Dictionary;
use std::collections::HashMap;

pub type PhraseRulesCollection<'p> = &'p HashMap<&'p str, PhraseContext<'p>>;
pub type ParsingRule<'p> = &'p [TemplateNode<'p>];

static mut PARSE_DP :Option<HashMap<(usize, String), Option<(SyntaxTreeNode, usize)>>> = None;

pub fn init_parse() {
    fit_rules::init_parse();
    unsafe {
        PARSE_DP = Some(HashMap::new());
    }
}

#[allow(dead_code)]
pub fn trim_front_iter<'t>(s :&'t [char])->(&'t [char], usize) {
    let mut i = 0;
    while !s.is_empty() && (s[i] == ' ' || s[i] == '\t' || s[i] == '\n' || s[i] == '\r') {
        i += 1;
    }
    (&s[i..], i)
}

pub fn parse<'p, 't>(s :&'t [char], part :ConcretePart<'t, 'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(SyntaxTreeNode, usize)> {
    let key = (s.len(), part.id.clone());
    unsafe {
        if let Some(k) = &PARSE_DP {
            if let Some(x) = k.get(&key) {
                return x.clone();
            }
        }
    }

    // let (s2, trims) :(&[char], usize) = trim_front_iter(s);
    let trims = 0;
    let mut m = trims;
    let mut mp = None;
    for r in part.rules {
        // println!("{}", fit_rules::parsingrule_tostr(&r.rules));
        // println!("{:20}{:10} : {}", part.id, r.name, &String::from_iter(s)[..].replace("\n", "\\n"));
        // if let Some((morphemes, x)) = fit_rules::fit_rules(&s2, &format!("{}@{}", part.part.name, r.name)[..], &r.rules, rules, dict, &part.cargs, &part.id) {
        if let Some((morphemes, x)) = fit_rules::fit_rules(&s, &format!("{}@{}", part.part.name, r.name)[..], &r.rules, rules, dict, &part.cargs, &part.id, r.allow_whitespace) {
            if x + trims > m {
                mp = Some(morphemes);
                m = x + trims;
            }
            // println!("Success");
        }
        else {
            // println!("Fail");
        }
    }

    if m == trims {
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                k.insert((s.len(), part.id), None);
            }
        }
        return None;
    }
    else {
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                if let Some(x) = k.get(&key) {
                    if let Some(x2) = x {
                        if x2.1 < m {
                            k.insert(key, Some((mp.clone().unwrap(), m)));
                        }
                    }
                }
            }
        }
        return Some((mp.unwrap(), m));
    }
}