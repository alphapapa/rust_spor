extern crate spor;

fn main() {
    spor::build_score_matrix("asdf", "asdf",
                             &spor::scoring::score_func,
                             &spor::scoring::gap_penalty);
}
