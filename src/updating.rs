use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};


use alignment::align::{Align, AlignmentCell};
use anchor::{Anchor, Context};
use scoring::{gap_penalty, score_func};

/// Update an anchor based on the current contents of its source file.
pub fn update(anchor: &Anchor, align: &Align) -> Result<Anchor, UpdateError> {
    let f = File::open(anchor.file_path())?;
    let handle = BufReader::new(f);
    _update(anchor, handle, align)
}

/// The main update implementation.
///
/// This takes in a reader of the anchored text, making it easier to test
/// (since it can work without the file actually existing).
fn _update(anchor: &Anchor, 
           mut anchor_file_reader: impl Seek + Read,
           align: &Align) -> Result<Anchor, UpdateError> {
    let ctxt = anchor.context();
    let mut full_text = String::new();
    anchor_file_reader.seek(SeekFrom::Start(0))?;
    anchor_file_reader.read_to_string(&mut full_text)?;

    let (_, alignments) = align(&ctxt.full_text(), &full_text, &score_func, &gap_penalty);

    let alignment = match alignments.first() {
        Some(a) => Ok(a),
        None => Err(UpdateError::NoAlignments)
    }?;

    let anchor_offset = (ctxt.offset() as usize) - ctxt.before().len();

    let source_indices: Vec<usize> = alignment
        .into_iter()
        .filter_map(|a| match a {
            AlignmentCell::Both { left: l, right: r } => Some((l, r)),
            _ => None,
        })
        .filter(|(a_idx, _)| index_in_topic(*a_idx + anchor_offset, &anchor))
        .map(|(_, s_idx)| *s_idx)
        .collect();

    let new_topic_offset = match source_indices.first() {
        Some(index) => Ok(index),
        None => Err(UpdateError::InvalidAlignment)
    }?;

    let context = Context::from_buf(
        anchor_file_reader,
        *new_topic_offset as u64,
        source_indices.len() as u64,
        anchor.context().width()
    )?;

    let updated = Anchor::new(
        anchor.file_path(),
        context,
        anchor.metadata().clone(),
        anchor.encoding().clone(),
    )?;

    Ok(updated)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateError {
    // No alignments could be found
    NoAlignments,

    // An alignment doesn't match the text
    InvalidAlignment,

    Io(std::io::ErrorKind, String),
}

impl From<std::io::Error> for UpdateError {
    fn from(err: std::io::Error) -> UpdateError {
        UpdateError::Io(err.kind(), err.description().to_string())
    }
}

// Determines if an index is in the topic of an anchor
fn index_in_topic(index: usize, anchor: &Anchor) -> bool {
    (index >= anchor.context().offset() as usize)
        && (index < anchor.context().offset() as usize + anchor.context().topic().len())
}

#[cfg(test)]
mod tests {
    extern crate ndarray;
    extern crate serde_yaml;

    use super::*;
    use super::super::alignment::smith_waterman::align;
    use std::path::PathBuf;
    use std::io::Cursor;

    #[test]
    fn successful_update() {
        let initial_text = "asdf";
        let final_text = "qwer\nasdf";

        let context = Context::from_buf(
            Cursor::new(initial_text.as_bytes()), 0, 4, 3).unwrap();

        let metadata = serde_yaml::from_str("foo: bar").unwrap();

        let anchor = Anchor::new(
            &PathBuf::from("/foo/bar"),
            context,
            metadata,
            "utf-8".to_string()).unwrap();

        let updated_anchor = _update(
            &anchor,
            Cursor::new(final_text.as_bytes()),
            &align).unwrap();
            
        assert_eq!(updated_anchor.context().offset(), 5);
    }
}
