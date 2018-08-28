use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use super::{CacheEntry, CacheResult, RefreshState};

lazy_static! {
    pub static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new());
}

/// A simple key-value store
pub struct Cache {
    map: HashMap<Arc<String>, CacheEntry>,
}

/// An overview of the entries in the cache, in a format that can be given to
/// a serializer.
#[derive(Serialize)]
pub struct CacheOverview {
    total_size: usize,
    list: Vec<OverviewEntry>,
}

/// An overview of the entries in
#[derive(Serialize)]
struct OverviewEntry {
    origin: &'static str,
    size: usize,
    name: Arc<String>,
    load_time: String,
}

/// Formats the `time` into a human-readable string in Texas-time(TM).
fn format_time(time: SystemTime) -> String {
    use chrono::offset::Local;
    use chrono::DateTime;

    let datetime: DateTime<Local> = time.into();
    format!("{}", datetime.format("%Y.%m.%d %T"))
}

impl Cache {

    /// Creates a new cache implementation
    pub fn new() -> Self {
        Cache {
            map: HashMap::new()
        }
    }

    /// Gets a serializable overview of the current state of the cache
    pub fn overview(&self) -> CacheOverview {
        use std::mem::size_of;

        let mut total_size = size_of::<Cache>();

        let list: Vec<OverviewEntry> = self.map.iter()
            .map(|(name, entry)| {

                // we take into account, all of the content AND the overhead
                // for the content of the cache
                let size = size_of::<CacheEntry>()
                    + entry.content.len()
                    + name.len();

                total_size += size;

                OverviewEntry {
                    origin: entry.origin.as_str(),
                    size,
                    name: name.clone(),
                    load_time: format_time(entry.load_time),
                }
            })
            .collect();

        CacheOverview {
            total_size,
            list,
        }
    }

//    pub fn put<S>(&mut self, key: S, value: S) -> Arc<String>
//    where
//        S: Into<String>
//    {
//        let (key, value) = (key.into(), value.into());
//
//        let entry = CacheEntry::from(value);
//        let value = entry.content.clone();
//        self.map.insert(Arc::new(key), entry);
//
//        value
//    }

    pub fn update<S>(&mut self, key: S, value: Arc<String>)
    where
        S: Into<String>, {
        self.transform(key, |_| value);
    }

    pub fn transform<S, F>(&mut self, key: S, action: F)
    where
        S: Into<String>,
        F: FnOnce(Arc<String>) -> Arc<String> {

        if let Some(entry) = self.map.get_mut(&key.into()) {
            entry.transform(action);
        }
    }

//    /// Attempts to read the entry with the given `key`. If it isn't in the cache,
//    /// then and Err will be returned.
//    ///
//    /// This method will ONLY ever return `Err` or `Cached`.
//    pub fn entry(&mut self, key: &String) -> CacheResult<Arc<String>, ()>
//    {
//        match self.map.get_mut(key) {
//            None => CacheResult::Err(()),
//            Some(value) => CacheResult::Cached(value.content.clone()),
//        }
//    }

    pub fn stripped_file<S>(&mut self, path: S) -> CacheResult<Arc<String>, io::Error>
    where
        S: Into<String>,
    {
        self.file_and_then(path, super::transform::strip_whitespace)
    }

//    pub fn file<S>(&mut self, path: S) -> CacheResult<Arc<String>, io::Error>
//    where
//        S: Into<String>,
//    {
//        self.file_and_then(path, |it| it)
//    }

    /// Gets the content of the file with the given `path` from the cache,
    /// loading the file into cache, if need be. This will also trigger a
    /// refresh of the cache entry, reloading the file if it has been modified
    /// since it was loaded into cache last.
    pub fn file_and_then<S, F>(
        &mut self,
        path: S,
        action: F
    ) -> CacheResult<Arc<String>, io::Error>
    where
        S: Into<String>,
        F: FnOnce(Arc<String>) -> Arc<String>,
    {
        let path = path.into();

        // if the item is in cache, then we'll just try to refresh it
        if let Some(entry) = self.map.get_mut(&path) {
            match entry.refresh() {
                RefreshState::Unnecessary => return CacheResult::Cached(entry.content.clone()),

                RefreshState::Success => {
                    entry.transform(action);
                    return CacheResult::New(entry.content.clone());
                },

                RefreshState::Failure(err) => return CacheResult::Err(err),

                // all files should be applicable for refreshing, which means
                // there's a conflicting key somewhere
                RefreshState::NotApplicable => {
                    eprintln!("Cache key conflict for `{}`", path);
                }
            }
        }

        // from here, we need to load the file into cache for the first time
        match CacheEntry::from_file(path.as_str().into()) {
            Ok(entry) => {
                let content = entry.content.clone();
                self.map.insert(Arc::new(path.clone()), entry);
                self.transform(path, action);
                CacheResult::New(content)
            },
            Err(err) => CacheResult::Err(err),
        }
    }
}
