pub mod alignment;
pub mod anchor;
pub mod scoring;

#[macro_use]
extern crate cute;
extern crate cpython;

use alignment::smith_waterman::*;
use cpython::*;

fn align(py: Python, a: String, b: String) -> PyResult<PyTuple> {
    let (score, alignments) = alignment::smith_waterman::align(
        &*a,
        &*b,
        &scoring::score_func,
        &scoring::gap_penalty);

    let alignments = c![
        c![
            match cell {
                AlignmentCell::Both {left, right} => (Some(left), Some(right)),
                AlignmentCell::RightGap {left} => (Some(left), None),
                AlignmentCell::LeftGap {right} => (None, Some(right)),
            },
            for cell in alignment
        ],
        for alignment in alignments
    ];

    let result = (score, alignments);

    Ok(result.to_py_object(py))
}

py_module_initializer!(rust_spor, initstatus, PyInit_rust_spor, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "align", py_fn!(py, align(a: String, b: String)))?;
    Ok(())
});
