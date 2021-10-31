use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

/// Wrapper for `Path` that asserts that the path is relative.
#[repr(transparent)]
pub(crate) struct RelPath {
    path: Path,
}

/// Wrapper for `PathBuf` that asserts that the path is relative.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RelPathBuf {
    path: PathBuf,
}

impl RelPath {
    pub(crate) fn new(path: &Path) -> &RelPath {
        assert!(
            !path.is_absolute(),
            "path must be relative: {}",
            path.display()
        );
        unsafe { &*(path as *const Path as *const RelPath) }
    }

    pub(crate) fn to_owned(&self) -> RelPathBuf {
        RelPathBuf {
            path: self.path.to_owned(),
        }
    }
}

impl RelPathBuf {
    pub(crate) fn _new(path: PathBuf) -> RelPathBuf {
        assert!(
            !path.is_absolute(),
            "path must be relative: {}",
            path.display()
        );
        RelPathBuf { path }
    }

    pub(crate) fn into_path_buf(self) -> PathBuf {
        self.path
    }
}

impl Deref for RelPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Deref for RelPathBuf {
    type Target = RelPath;

    fn deref(&self) -> &Self::Target {
        RelPath::new(&self.path)
    }
}
