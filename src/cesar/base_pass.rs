use crate::cesar::{language::PropLang};
use crate::cesar::config;
use egg::*;
use log::debug;
use log::info;

pub trait BasePass {
    
    fn get_assumptions(&self) -> &str;

    fn make_rules() -> Vec<Rewrite<PropLang, ()>>;

    fn get_runner(has_node_limit: bool) -> Runner<PropLang, ()> {
        let mut runner Runner::<PropLang, ()>::default();

        if has_node_limit {
            runner
                .with_node_limit(100_000)
                .with_iter_limit(100_000)
        } else {
            runner
        }
    }

    fn simplify(problem: String, assumptions: String, has_node_limit: bool, timeout: u64) -> String {
        debug("Running simplify with {0}", has_node_limit);
        unsafe { ASSUMPTIONS = assumptions };
        
        let problem = problem.parse().unwrap();
        let rules = Self::make_rules();

        let mut runner = Self::get_runner(has_node_limit)
            .with_time_limit(std::time::Duration::from_secs(timeout))
            .with_explanations_enabled()
            .with_expr(&problem).run(&rules);

        let extractor = Extractor::new(&runner.egraph, AstSize);
        let simplified = extractor.find_best(runner.roots[0]);
        debug!("Start of Pass debug info");
        debug!("Simplification finished");
        debug!("Original Problem: {}", problem);
        debug!("Simplifed Problem: {}", simplified.1);
        debug!("Cost of Simplification: {}", simplified.0);
        debug!("Runner stop reason: {:?}", runner.stop_reason);

        let explanation = runner.explain_equivalence(&problem, &simplified.1).get_flat_string();
        debug!("Explanations: {}", explanation);
        debug!("End of Pass debug info");

};
