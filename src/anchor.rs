use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Result, Seek, SeekFrom};
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
    pub fn new(
        file_path: &Path,
        offset: u64,
        width: u64,
        context_width: u64,
    ) -> Result<Context> {
        let f = File::open(file_path)?;
        let handle = &mut BufReader::new(f);

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
                "Anchor file path's must be absolute"))
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
