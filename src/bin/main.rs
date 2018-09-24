extern crate ordered_float;
extern crate spor;

const INPUT1: &str = "GGTTGACTA";
const INPUT2: &str = "TGTTACGG";


fn main() {
    let (score_matrix, traceback_matrix) = spor::alignment::smith_waterman::build_score_matrix(
        INPUT1, INPUT2,
        &spor::scoring::score_func,
        &spor::scoring::gap_penalty);

    let max_idx = score_matrix.indexed_iter()
        .max_by_key(|n| ordered_float::OrderedFloat(*n.1))
        .unwrap().0;

    println!("max_idx: {:?}", max_idx);
    println!("score_matrix: {:?}", score_matrix);
    println!("tb_matrix: {:?}", traceback_matrix);

    let xxx = spor::alignment::smith_waterman::tracebacks(&traceback_matrix, max_idx);
    println!("tracebacks: {:?}", xxx);
}
