use std::collections::HashMap;
use syntax::codemap::FileLoader;
use std::path::{Path, PathBuf};
use std::io;

pub(crate) struct InMemoryFileLoader<T: FileLoader> {
    files: HashMap<PathBuf, String>,
    inner_file_loader: T,
}

impl<T: FileLoader> InMemoryFileLoader<T> {
    pub(crate) fn new(inner: T) -> InMemoryFileLoader<T> {
        InMemoryFileLoader {
            files: HashMap::new(),
            inner_file_loader: inner,
        }
    }

    pub(crate) fn add_file<P: Into<PathBuf>>(&mut self, file_name: P, contents: String) {
        self.files.insert(file_name.into(), contents);
    }
}

impl<T: FileLoader> FileLoader for InMemoryFileLoader<T> {
    fn file_exists(&self, path: &Path) -> bool {
        self.files.contains_key(path) || self.inner_file_loader.file_exists(path)
    }

    fn abs_path(&self, _: &Path) -> Option<PathBuf> {
        None
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        if let Some(contents) = self.files.get(path) {
            return Ok(contents.to_string());
        }
        self.inner_file_loader.read_file(path)
    }
}
