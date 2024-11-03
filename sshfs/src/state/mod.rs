use std::sync::Arc;

use page::Page;
use tokio::sync::Mutex;

pub mod fs_manager;
pub mod fs_tree;
pub mod page;

#[derive(Debug)]
pub struct State {
    exit: std::sync::atomic::AtomicBool,
    render_notifier: Arc<tokio::sync::Notify>,
    page: Mutex<Page>,
    local_fs: Arc<fs_tree::FsTree>,
    remote_fs: Arc<fs_tree::FsTree>,
}

// reading state, this must be blocking
impl State {
    pub fn is_exiting(&self) -> bool {
        self.exit.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn notified(self: Arc<Self>) {
        let wait_notifier = self.render_notifier.clone();
        let future = wait_notifier.notified();
        futures::executor::block_on(future);
    }
}

// update state
impl State {
    pub fn exit(&self) {
        self.update(move || {
            self.exit.store(true, std::sync::atomic::Ordering::Relaxed);
        });
    }
}

// utility functions
impl State {
    pub fn new(local_fs: fs_tree::FsTree, remote_fs: fs_tree::FsTree) -> Arc<Self> {
        let this = Arc::new(Self {
            exit: std::sync::atomic::AtomicBool::new(false),
            render_notifier: Arc::new(tokio::sync::Notify::new()),
            page: Mutex::new(Page::Main),
            local_fs: Arc::new(local_fs),
            remote_fs: Arc::new(remote_fs),
        });

        this.update(|| {});

        this
    }

    fn update(&self, update_state_func: impl FnOnce()) {
        update_state_func();
        self.render_notifier.notify_one();
    }
}
