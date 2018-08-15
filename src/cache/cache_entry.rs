use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use super::Origin;
use super::RefreshState;

/// An entry in the cache system's map.
pub struct CacheEntry {

    /// How was this entry generated?
    pub origin: Origin,

    /// When was this entry generated?
    pub load_time: SystemTime,

    /// The content of the entry
    pub content: Arc<String>,
}

/// Reads the file
fn read_file(path: &PathBuf) -> io::Result<Arc<String>> {
    use std::io::Read;
    use std::fs::File;

    let mut content = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut content)?;
    Ok(Arc::new(content))
}

impl CacheEntry {

    /// Loads the file at `path` into a new cache entry.
    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        let content = read_file(&path)?;

        Ok(CacheEntry {
            origin: Origin::File(path),
            load_time: SystemTime::now(),
            content,
        })
    }

//    /// Creates a new cache entry directly using the given source.
//    pub fn from(data: String) -> CacheEntry {
//        let content = Arc::new(data);
//
//        CacheEntry {
//            origin: Origin::Manual,
//            load_time: SystemTime::now(),
//            content,
//        }
//    }

    /// Transforms the contents of this entry, without modifying the load time
    /// or origin.
    pub fn transform<F>(&mut self, action: F)
        where F: FnOnce(Arc<String>) -> Arc<String> {

        self.content = action(self.content.clone());
    }

    /// Refreshes the data if it's from an origin which can be refreshed.
    ///
    /// Unlike [transform], this *will* update the [load_time] of the cache
    /// entry.
    pub fn refresh(&mut self) -> RefreshState {
        let (content, state) = match self.origin {
            Origin::File(ref path) =>
                refresh_file(self.load_time, path),

//            Origin::Manual => (None, RefreshState::NotApplicable),
        };

        // if the refresh was a success, then we'll update our content and
        // timestamp
        if state == RefreshState::Success {
            self.load_time = SystemTime::now();
            self.content = content.expect("Success state didn't give string!");
        }

        state
    }
}

/// Component function belonging to [refresh]. This will update the contents
/// of this CacheEntry, but only if we can get the file's metadata for its
/// last modification time, AND that data says we shouldn't update.
///
/// PRECONDITIONS:
/// 1. `origin` **must be** [Origin::File].
fn refresh_file(
    load_time: SystemTime,
    path: &PathBuf
) -> (Option<Arc<String>>, RefreshState) {
    use std::fs;

    // update will ONLY be false if we *successfully* get the
    // metadata for the file's last update time *AND* if that says
    // we shouldn't update. ALL OTHER CASES YIELD TRUE
    let update = fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        // if the metadata says we loaded the file before it was
        // edited, then we need to reload
        .map(|modified| load_time < modified)
        // if there was an error along the line, then we need to
        // reload
        .unwrap_or(true);

    if !update {
        return (None, RefreshState::Unnecessary);
    }

    // try to update the content, and we might succeed or not
    read_file(path)
        .map(|content| (Some(content), RefreshState::Success))
        .unwrap_or_else(|err| (None, RefreshState::Failure(err)))
}
