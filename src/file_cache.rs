use std::collections::HashMap;
use std::time::SystemTime;
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref CACHE: Mutex<FileCache> = Mutex::new(FileCache::new());
}

struct CacheItem {
    modified_time: SystemTime,
    load_time: SystemTime,
    contents: Arc<String>,
}

pub struct FileCache {
    map: HashMap<PathBuf, CacheItem>,
}

#[derive(Serialize)]
pub struct CacheOverviewEntry {
    pub size: usize,
    pub file: String,
    pub modified_time: String,
    pub load_time: String,
}

fn convert_time(system: SystemTime) -> String {
    use chrono::offset::Local;
    use chrono::DateTime;

    let datetime: DateTime<Local> = system.into();
    format!("{}", datetime.format("%Y.%m.%d %T"))
}

impl FileCache {

    pub fn new() -> Self {
        FileCache {
            map: HashMap::new(),
        }
    }

    pub fn overview(&self) -> Vec<CacheOverviewEntry> {
        self.map.iter()
            .map(|(path, item)| {
                let file = path.to_str().map(String::from)
                    .expect("Failed to get path name");

                CacheOverviewEntry {
                    size: item.contents.len(),
                    file,
                    modified_time: convert_time(item.modified_time.clone()),
                    load_time: convert_time(item.load_time.clone()),
                }
            })
            .collect()
    }

    pub fn get<P: Into<PathBuf>>(&mut self, path: P) -> Option<Arc<String>> {
        use std::io::Read;

        let path = path.into();
        let last_modified = fs::metadata(&path).ok()?.modified().ok()?;

        // if the entry already exists, then we'll return the contents if they
        // haven't been changed
        if let Some(item) = self.map.get(&path) {

            // if the file hasn't been modified, then we don't need to reload it
            if item.modified_time >= last_modified {
                return Some(item.contents.clone());
            }

            // otherwise, we need to reload it
        }

        // let's try to read the file then
        let mut content = String::new();
        let mut file = File::open(&path).ok()?;
        file.read_to_string(&mut content).ok()?;
        let content = Arc::new(content);

        let info = CacheItem {
            modified_time: last_modified,
            load_time: SystemTime::now(),
            contents: content.clone(),
        };

        // now, put that content into our map
        self.map.insert(path, info);

        Some(content)
    }

}
