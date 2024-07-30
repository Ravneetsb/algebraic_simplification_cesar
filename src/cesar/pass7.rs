use crate::cesar::language::PropLang;
use crate::cesar::base;
use crate::cesar::config;
use egg::*;

/// This pass does multiplication distribution, i.e. the transformation x*a + x*b = x*(a+b).
pub struct Pass7;

pub static mut ASSUMPTIONS: String =  String::new();

impl Pass7 {

    // reference: https://docs.rs/egg/latest/egg/macro.rewrite.html.
    fn make_rules() -> Vec<Rewrite<PropLang, ()>> {
        vec![
            // Multiplication distribution.
            rewrite!("mul-dist"; "(+ (* ?x ?y) (* ?x ?z))" => "(* ?x (+ ?y ?z))"),
            rewrite!("mul-dist-minus"; "(- (* ?x ?y) (* ?x ?z))" => "(* ?x (- ?y ?z))"),
            // Multiplication commutes.
            rewrite!("mul-comm"; "(* ?x ?y)" => "(* ?y ?x)"),
            // Multiplication associates.
            rewrite!("mul-assoc"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),
            // x+(-y) is just x-y.
            rewrite!("minus-rewrite"; "(+ (- ?x) ?y)" => "(- ?y ?x)"),
            rewrite!("minus-rewrite-2"; "(+ ?x (- ?y))" => "(- ?x ?y)"),
            // Get minus outside multiplication.
            rewrite!("minus-distribute"; "(* (- ?x) ?y)" => "(- (* ?y ?x))"),
            // Times 1 is just the number.
            rewrite!("times-one"; "(* ?x 1)" => "?x"),
        ]
    }

    pub fn simplify(problem: String, assumptions: String) -> String {
        unsafe { ASSUMPTIONS = assumptions };
        
        base::simplify(problem, true, config::LONG_TIMEOUT, Self::make_rules())
    }
}
