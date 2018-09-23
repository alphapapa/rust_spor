extern crate spor;

// TODO: These are also used in the tests. Put them in some shared places like "spor::scoring".
fn score_func(a: char, b: char) -> f32 {
    if a == b {
        3.0
    } else {
        -3.0
    }
}

fn gap_penalty(gap: u32) -> f32 {
    if gap == 1 {
        2.0
    } else {
        (gap as f32) * gap_penalty(1)
    }
}

fn main() {
    spor::build_score_matrix("asdf", "asdf", &score_func, &gap_penalty);
}
