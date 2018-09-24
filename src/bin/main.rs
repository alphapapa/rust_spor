extern crate ordered_float;
extern crate spor;

fn main() {
    let (score_matrix, traceback_matrix) = spor::build_score_matrix(
        "asdf", "asdf",
        &spor::scoring::score_func,
        &spor::scoring::gap_penalty);

    let max_idx = score_matrix.indexed_iter()
        .max_by_key(|n| ordered_float::OrderedFloat(*n.1))
        .unwrap().0;

    println!("max_idx: {:?}", max_idx);
    println!("score_matrix: {:?}", score_matrix);
    println!("tb_matrix: {:?}", traceback_matrix);

    let xxx = spor::tracebacks(&traceback_matrix, max_idx);
    println!("tracebacks: {:?}", xxx);
}
