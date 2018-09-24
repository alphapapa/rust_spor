extern crate spor;

fn main() {
    let (scores, _tbs) = spor::build_score_matrix(
        "asdf", "asdf",
        &spor::scoring::score_func,
        &spor::scoring::gap_penalty);

    for (idx, val) in scores.indexed_iter() {
        println!("idx: {:?}, val: {:?}", idx, val);
    }
}
