// A standard scoring function
pub fn score_func(a: char, b: char) -> f32 {
    if a == b {
        3.0
    } else {
        -3.0
    }
}

// A standard gap-penalty function
pub fn gap_penalty(gap: u32) -> f32 {
    if gap == 1 {
        2.0
    } else {
        (gap as f32) * gap_penalty(1)
    }
}

