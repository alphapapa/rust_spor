use std::fs::File;
use std::io::{BufReader, Read};

use alignment::smith_waterman::{align, AlignmentCell};
use anchor::Anchor;
use result::{from_str, Result};
use scoring::{gap_penalty, score_func};

// Determines if an index is in the topic of an anchor
fn index_in_topic(index: usize, anchor: &Anchor) -> bool {
    (index >= anchor.context.offset as usize) && (index < anchor.context.offset as usize + anchor.context.topic.len())
}

// Update an anchor based on the current contents of its source file.
pub fn update(anchor: &Anchor) -> Result<Anchor> {
    let f = File::open(&anchor.file_path)?;
    let ctxt = &anchor.context;
    let mut handle = BufReader::new(f);
    let mut full_text = String::new();
    handle.read_to_string(&mut full_text)?;

    let (_, alignments) = align(&ctxt.full_text(), &full_text, &score_func, &gap_penalty);

    if alignments.is_empty() {
        return from_str("No alignments found")
    }

    let alignment = match alignments.first() {
        Some(a) => Ok(a),
        None => from_str("No alignments found")
    }?;

    let anchor_offset = (ctxt.offset as usize) - ctxt.before.len();

    let source_indices: Vec<usize> = alignment.into_iter()
        .filter_map(|a| match a {
            AlignmentCell::Both{left: l, right: r} => Some((l, r)),
            _ => None
        })
        .filter(|(a_idx, _)| index_in_topic(*a_idx + anchor_offset, &anchor))
        .map(|(_, s_idx)| *s_idx)
        .collect()
        ;

    if source_indices.is_empty() {
        return from_str("Best alignment does map topic to updated source.")
    }

    let updated = Anchor::new(
        &anchor.file_path,
        source_indices[0] as u64,
        source_indices.len() as u64,
        anchor.context.width,
        anchor.metadata.clone(),
        anchor.encoding.clone())?;
    
    Ok(updated)
}