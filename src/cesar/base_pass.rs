use crate::cesar::{language::PropLang};
use crate::cesar::config;
use egg::*;
use log::debug;
use log::info;

pub static mut ASSUMPTIONS: String = String::new();

pub trait BasePass {
    fn make_rules() -> Vec<Rewrite<PropLang, ()>>

    fn get_runner(has_node_limit: bool) -> Runner<PropLang, ()> {
        let mut runner Runner::<PropLang, ()>::default();

        if has_node_limit {
            runner
                .with_node_limit(100_000)
                .with_iter_limit(100_000)
        } else {
            runner
        }
};
