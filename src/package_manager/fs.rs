use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self, FileType};
use std::path::{Path, PathBuf};

use log::debug;

use crate::{Error, IoError, Result};

type DirEntry = (PathBuf, FileType, OsString);

thread_local! {
    static IS_FILE_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    static IS_DIR_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    static READ_FILE_CACHE: RefCell<HashMap<PathBuf, String>> = RefCell::new(HashMap::new());
    static READ_DIR_CACHE: RefCell<HashMap<PathBuf, Vec<DirEntry>>> = RefCell::new(HashMap::new());
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

pub fn read_with_cache(path: &Path) -> Result<String> {
    debug!("read with cache start...");

    if let Some(cached) = READ_FILE_CACHE.with(|cache| cache.borrow().get(path).cloned()) {
        debug!("read with cache end, cached");
        return Ok(cached);
    }

    let content = fs::read_to_string(path).map_err(|source| IoError::new("read", path, source))?;
    READ_FILE_CACHE.with(|cache| {
        cache.borrow_mut().insert(path.to_owned(), content.clone());
    });

    debug!("read with cache end");
    Ok(content)
}

pub fn read_dir_with_cache(path: &Path) -> Result<Vec<(PathBuf, FileType, OsString)>> {
    debug!("read dir with cache start...");

    if let Some(cached) = READ_DIR_CACHE.with(|cache| cache.borrow().get(path).cloned()) {
        debug!("read dir with cache end, cached");
        return Ok(cached);
    }

    let entries = fs::read_dir(path)
        .map_err(|source| IoError::new("read directory", path, source))?
        .map(|entry| {
            let entry = entry.map_err(|source| {
                Error::from(IoError::new("read directory entry", path, source))
            })?;
            let entry_path = entry.path();
            let file_type = entry.file_type().map_err(|source| {
                Error::from(IoError::new(
                    "read directory entry type",
                    entry_path.to_owned(),
                    source,
                ))
            })?;
            Ok((entry_path, file_type, entry.file_name()))
        })
        .collect::<Result<Vec<_>>>()?;
    READ_DIR_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .insert(path.to_owned(), entries.to_owned());
    });

    debug!("read dir with cache end, entries: {:?}", entries);
    Ok(entries)
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
