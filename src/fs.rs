use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::{self, FileType};
use std::path::{Path, PathBuf};

use anyhow::Result;
use log::debug;

type DirEntry = (PathBuf, FileType, OsString);

thread_local! {
    static IS_FILE_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    static IS_DIR_CACHE: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    static READ_FILE_CACHE: RefCell<HashMap<PathBuf, String>> = RefCell::new(HashMap::new());
    static READ_DIR_CACHE: RefCell<HashMap<PathBuf, Vec<DirEntry>>> = RefCell::new(HashMap::new());
}

pub fn is_file_with_cache(path: &Path) -> bool {
    debug!("is file with cache start...");

    if let Some(cached) = IS_FILE_CACHE.with(|cache| cache.borrow().get(path).cloned()) {
        debug!("is file with cache end, cached: {}", cached);
        return cached;
    }

    let is_file = path.is_file();
    IS_FILE_CACHE.with(|cache| {
        cache.borrow_mut().insert(path.to_owned(), is_file);
    });

    debug!("is file with cache end, is_file: {}", is_file);
    is_file
}

pub fn is_dir_with_cache(path: &Path) -> bool {
    debug!("is dir with cache start...");
    if let Some(cached) = IS_DIR_CACHE.with(|cache| cache.borrow().get(path).cloned()) {
        debug!("is dir with cache end, cached: {}", cached);
        return cached;
    }

    let is_dir = path.is_dir();
    IS_DIR_CACHE.with(|cache| {
        cache.borrow_mut().insert(path.to_owned(), is_dir);
    });

    debug!("is dir with cache end, is_dir: {}", is_dir);
    is_dir
}

pub fn read_with_cache(path: &Path) -> Result<String> {
    debug!("read with cache start...");

    if let Some(cached) = READ_FILE_CACHE.with(|cache| cache.borrow().get(path).cloned()) {
        debug!("read with cache end, cached");
        return Ok(cached);
    }

    let content = fs::read_to_string(path)?;
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

    let entries = fs::read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            Ok((entry.path(), entry.file_type()?, entry.file_name()))
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
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    debug!("cwd: {:?}", cwd);

    std::iter::successors(Some(cwd), |path| path.parent().map(PathBuf::from))
}
