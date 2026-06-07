use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::FileType;
use std::path::{Path, PathBuf};

use log::debug;

type DirEntry = (PathBuf, Option<FileType>, OsString);

thread_local! {
    static IS_FILE_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    static IS_DIR_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    static READ_FILE_CACHE: RefCell<HashMap<PathBuf, Option<String>>> = RefCell::new(HashMap::new());
    static READ_DIR_CACHE: RefCell<HashMap<PathBuf, Option<Vec<DirEntry>>>> = RefCell::new(HashMap::new());
}

pub fn is_file_with_cache(path: &Path) -> bool {
    IS_FILE_CACHE.with(|cache| {
        debug!("is file with cache start...");

        let mut map = cache.borrow_mut();
        if let Some(cached) = map.get(path) {
            debug!("is file with cache end, cached: {:?}", cached);
            return *cached;
        }

        let is_file = path.is_file();
        map.insert(path.to_owned(), is_file);

        debug!("is file with cache end, is_file: {:?}", is_file);
        is_file
    })
}

pub fn is_dir_with_cache(path: &Path) -> bool {
    IS_DIR_CACHE.with(|cache| {
        debug!("is dir with cache start...");

        let mut map = cache.borrow_mut();
        if let Some(cached) = map.get(path) {
            debug!("is dir with cache end, cached: {:?}", cached);
            return *cached;
        }

        let is_dir = path.is_dir();
        map.insert(path.to_owned(), is_dir);

        debug!("is dir with cache end, is_dir: {:?}", is_dir);
        is_dir
    })
}

pub fn read_with_cache(path: &Path) -> Option<String> {
    READ_FILE_CACHE.with(|cache| {
        debug!("read with cache start...");

        let mut map = cache.borrow_mut();
        if let Some(cached) = map.get(path) {
            debug!("read with cache end, cached: {:?}", cached);
            return cached.to_owned();
        }

        if !is_file_with_cache(path) {
            debug!("read with cache end, not found");
            return None;
        }

        let content = std::fs::read_to_string(path).ok();
        map.insert(path.to_owned(), content.to_owned());

        debug!("read with cache end, content: {:?}", content);
        content
    })
}

pub fn read_dir_with_cache(path: &Path) -> Option<Vec<(PathBuf, Option<FileType>, OsString)>> {
    READ_DIR_CACHE.with(|cache| {
        debug!("read dir with cache start...");

        let mut map = cache.borrow_mut();
        if let Some(cached) = map.get(path) {
            debug!("read dir with cache end, cached: {:?}", cached);
            return cached.to_owned();
        }

        if !is_dir_with_cache(path) {
            debug!("read dir with cache end, not found");
            return None;
        }

        let entries: Vec<_> = std::fs::read_dir(path).ok()?.flatten().collect();
        let entries: Vec<_> = entries
            .iter()
            .map(|entry| (entry.path(), entry.file_type().ok(), entry.file_name()))
            .collect();
        map.insert(path.to_owned(), Some(entries.to_owned()));

        debug!("read dir with cache end, entries: {:?}", entries);
        Some(entries)
    })
}

pub fn walk_up(cwd: Option<&Path>) -> impl Iterator<Item = PathBuf> {
    debug!("walk up start...");

    let cwd = cwd
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    debug!("cwd: {:?}", cwd);

    std::iter::successors(Some(cwd), |path| path.parent().map(PathBuf::from))
}
