use std::{
    future::Future,
    path::{Path, PathBuf},
    process::Output,
};

pub trait FsManager {
    fn get_file_metadata(
        &self,
        path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<Metadata, anyhow::Error>>;

    fn get_children(
        &self,
        path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<Vec<Metadata>, anyhow::Error>>;

    fn get_parent(
        &self,
        path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<Metadata, anyhow::Error>>;
}

#[derive(Debug, Default)]
pub struct Metadata {
    pub size: u64,
    pub created_at: u64,
    pub name: String,
    pub is_dir: bool,
    pub path: PathBuf,
}

impl AsRef<Path> for &Metadata {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
