extern crate diff;

use super::anchor::{Anchor, Context};
use super::repository::*;
use super::result::Result;
use std::path::Path;

fn full_text(context: &Context) -> String {
    let mut text = vec![];
    text.append(&mut context.before.clone());
    text.push(context.line.clone());
    text.append(&mut context.after.clone());
    text.join("")
}

type Diff = Vec<String>;

fn context_diff(_file_name: &Path, c1: &Context, c2: &Context) -> Diff {
    let c1_text = full_text(c1);
    let c2_text = full_text(c2);

    let mut result = vec![];
    for diff in diff::lines(&c1_text, &c2_text) {
        match diff {
            diff::Result::Left(l) => result.push(format!("- {}", l)),
            diff::Result::Right(r) => result.push(format!("+ {}", r)),
            _ => (),
        };
    }

    result
}

pub fn validate(repo: &Repository) -> Vec<Result<(AnchorId, std::path::PathBuf, Diff)>> {
    let mut result = vec![];

    for r in repo {
        match r {
            Err(msg) => {
                result.push(Err(msg));
            }
            Ok((id, anchor)) => {
                let context_size =
                    std::cmp::max(anchor.context.before.len(), anchor.context.after.len());

                let new_anchor = Anchor::new(
                    context_size,
                    &anchor.file_path,
                    anchor.line_number,
                    anchor.metadata.clone(),
                    anchor.columns.clone(),
                    &repo.root,
                );

                match new_anchor {
                    Err(err) => {
                        result.push(Err(err.into()));
                    }
                    Ok(new_anchor) => {
                        assert!(anchor.file_path == new_anchor.file_path);
                        assert!(anchor.line_number == new_anchor.line_number);
                        assert!(anchor.columns == new_anchor.columns);
                        assert!(anchor.metadata == new_anchor.metadata);

                        let diff = context_diff(
                            &repo.root.join(&anchor.file_path),
                            &anchor.context,
                            &new_anchor.context,
                        );

                        if diff.len() > 0 {
                            result.push(Ok((id, anchor.file_path, diff)));
                        }
                    }
                }
            }
        }
    }

    result
}
