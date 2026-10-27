//! Source-declared Solaris failure for gopkg.in/fsnotify.v1 1.4.7.
// Copyright The Go Authors. BSD-3-Clause; see ../FSNOTIFY-V1-LICENSE.
// The pinned Agones utility imports this dependency, not fsnotify 1.9.0.
// Its NewWatcher unconditionally returns this error before creating a port.
use super::*;

#[cgp_provider(FileWatcherOpenerComponent)]
impl FileWatcherOpener<UtilityRuntimeContext> for OpenNativeFileWatcher {
    fn open_file_watcher(_context: &UtilityRuntimeContext, _path: &Path) -> Result<Arc<NativeWatch>, BoxError> {
        Err(Errors::default().New("FEN based watcher not yet supported for fsnotify\n"))
    }
}
