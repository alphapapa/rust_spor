use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Result, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    pub before: String,
    pub offset: u64,
    pub topic: String,
    pub after: String,
    pub width: u64,
}

impl Context {
    fn new(
        handle: &mut BufReader<std::fs::File>,
        offset: u64,
        width: u64,
        context_width: u64,
    ) -> Result<Context> {
        // read topic
        handle.seek(SeekFrom::Start(offset))?;

        let mut topic = String::new();
        handle.take(width).read_to_string(&mut topic)?;

        if topic.len() < width as usize {
            return Err(Error::new(ErrorKind::InvalidInput, "Unable to read topic"));
        }

        // read before
        let before_offset = max(0, offset - context_width);
        let before_width = offset - before_offset;
        handle.seek(SeekFrom::Start(before_offset))?;
        let mut before = String::new();
        handle.take(before_width).read_to_string(&mut before)?;
        if before.len() < before_width as usize {
            return Err(Error::new(ErrorKind::InvalidInput, "Unable to read before"));
        }

        // read after
        let after_offset = offset + width;
        handle.seek(SeekFrom::Start(after_offset))?;
        let mut after = String::new();
        handle.take(context_width).read_to_string(&mut after)?;

        let context = Context {
            before: before,
            offset: offset,
            topic: topic,
            after: after,
            width: context_width,
        };

        Ok(context)
    }

    pub fn full_text(self: &Context) -> String {
        format!("{}{}{}", self.before, self.topic, self.after)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Anchor {
    pub file_path: PathBuf,
    pub encoding: String, // TODO: Is there some "encoding" type?
    pub context: Context,
    pub metadata: serde_yaml::Value,
}

impl Anchor {
    pub fn new(
        file_path: &Path,
        offset: u64,
        width: u64,
        context_width: u64,
        metadata: serde_yaml::Value,
        encoding: String,
    ) -> std::io::Result<Anchor> {
        if !file_path.is_absolute() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Anchor file path's must be absolute"))
        }

        let f = File::open(file_path)?;
        let mut handle = BufReader::new(f);

        let context = Context::new(&mut handle, offset, width, context_width)?;

        let anchor = Anchor {
            file_path: PathBuf::from(file_path),
            encoding: encoding,
            context: context,
            metadata: metadata,
        };

        Ok(anchor)
    }
}
