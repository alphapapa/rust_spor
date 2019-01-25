extern crate diff;

use anchor::Anchor;
use std::io::Result;

pub fn get_anchor_diff(anchor: &Anchor) -> Result<(bool, Vec<String>)> {
    let new_anchor = Anchor::new(
        &anchor.file_path,
        anchor.context.offset,
        anchor.context.topic.len() as u64,
        anchor.context.width,
        anchor.metadata.clone(),
        anchor.encoding.clone(),
    )?;

    let mut diff_strings: Vec<String> = Vec::new();

    let mut changed = false;

    for diff in diff::lines(
        anchor.context.full_text().as_str(),
        new_anchor.context.full_text().as_str(),
    ) {
        changed = match diff {
            diff::Result::Both(_, _) => changed,
            _ => true
        };

        let diff_text = match diff {
            diff::Result::Left(l) => format!("-{}", l),
            diff::Result::Both(l, _) => format!(" {}", l),
            diff::Result::Right(r) => format!("+{}", r),
        };
        diff_strings.push(diff_text);
    }

    Ok((changed, diff_strings))
}
