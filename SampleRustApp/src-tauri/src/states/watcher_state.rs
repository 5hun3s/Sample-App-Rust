use std::sync::Mutex;

use notify::RecommendedWatcher;

pub struct WatcherState {
    watcher: Mutex<Option<RecommendedWatcher>>,
}

impl WatcherState {
    pub fn new() -> Self {
        Self {
            watcher: Mutex::new(None),
        }
    }

    pub fn set_watcher(&self, watcher: RecommendedWatcher) -> Result<(), String> {
        let mut guard = self
            .watcher
            .lock()
            .map_err(|_| "監視状態のロックに失敗しました。".to_string())?;

        /*
         * すでに監視中だった場合、古いWatcherはここでdropされる。
         * dropされると古い監視も停止する。
         */
        *guard = Some(watcher);

        Ok(())
    }

    pub fn stop(&self) -> Result<bool, String> {
        let mut guard = self
            .watcher
            .lock()
            .map_err(|_| "監視状態のロックに失敗しました。".to_string())?;

        let was_running = guard.is_some();

        /*
         * OptionをNoneにするとRecommendedWatcherがdropされ、
         * フォルダ監視が停止する。
         */
        *guard = None;

        Ok(was_running)
    }

    pub fn is_running(&self) -> Result<bool, String> {
        let guard = self
            .watcher
            .lock()
            .map_err(|_| "監視状態のロックに失敗しました。".to_string())?;

        Ok(guard.is_some())
    }
}
