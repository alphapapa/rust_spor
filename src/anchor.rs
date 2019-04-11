use std::cmp::max;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    before: String,
    offset: u64,
    topic: String,
    after: String,
    width: u64,
}

impl Context {
    pub fn new(text: &str, offset: u64, width: u64, context_width: u64) -> Result<Context> {
        let topic: String = text
            .chars()
            .skip(offset as usize)
            .take(width as usize)
            .collect();

        if topic.len() < width as usize {
            return Err(Error::new(ErrorKind::InvalidInput, "Unable to read topic"));
        }

        // read before
        let before_offset = if context_width <= offset {
            max(0, offset - context_width)
        } else {
            0
        };
        let before_width = offset - before_offset;
        let before: String = text
            .chars()
            .skip(before_offset as usize)
            .take(before_width as usize)
            .collect();

        // read after
        let after_offset = offset + width;
        let after_width = after_offset + context_width;
        let after: String = text
            .chars()
            .skip(after_offset as usize)
            .take(after_width as usize)
            .collect();

        let context = Context {
            before: before,
            offset: offset,
            topic: topic,
            after: after,
            width: context_width,
        };

        Ok(context)
    }

    pub fn before(&self) -> &String {
        &self.before
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn topic(&self) -> &String {
        &self.topic
    }

    pub fn after(&self) -> &String {
        &self.after
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn full_text(self: &Context) -> String {
        format!("{}{}{}", self.before, self.topic, self.after)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Anchor {
    file_path: PathBuf,
    encoding: String,
    context: Context,
    metadata: serde_yaml::Value,
}

impl Anchor {
    pub fn new(
        file_path: &Path,
        context: Context,
        metadata: serde_yaml::Value,
        encoding: String,
    ) -> std::io::Result<Anchor> {
        if !file_path.is_absolute() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Anchor file path's must be absolute",
            ));
        }

        let anchor = Anchor {
            file_path: PathBuf::from(file_path),
            encoding: encoding,
            context: context,
            metadata: metadata,
        };

        Ok(anchor)
    }

    pub fn file_path(&self) -> &PathBuf {
        return &self.file_path;
    }

    pub fn encoding(&self) -> &String {
        return &self.encoding;
    }

    pub fn context(&self) -> &Context {
        return &self.context;
    }

    pub fn metadata(&self) -> &serde_yaml::Value {
        return &self.metadata;
    }
}

#[cfg(test)]
mod tests {
    extern crate ndarray;
    extern crate serde_yaml;

    use super::*;

    mod context {
        use super::*;

        #[test]
        fn construct_context_with_topic_at_front_of_file() {
            Context::new("text", 0, 4, 3).unwrap();
        }
    }
}
