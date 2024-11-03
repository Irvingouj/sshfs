use sshfs::{
    logs::init_file_log,
    sshfs::tokio_fs_manager::TokioFsManager,
    state::{self, State},
    ui::app::App,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _log_guard = init_file_log(&None);
    info!("Starting sshfs");
    let local_fs = TokioFsManager::new();
    let local_fs = state::fs_tree::FsTree::new(".", &local_fs).await?;

    //     let remote_fs = TokioFsManager::new();
    let remote_fs = TokioFsManager::new();
    let remote_fs = state::fs_tree::FsTree::new(".", &remote_fs).await?;
    let state = State::new(local_fs, remote_fs);


    info!(?state, "State created");
    App::run(state).await?;

    Ok(())
}
