use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_files() {
        let mut fs = file_or_dirfiles(Path::new("data/"), None).unwrap();
        fs.sort();

        assert_eq!(
            fs,
            vec!(
                Path::new("data/file1.fasta"),
                Path::new("data/file1.txt"),
                Path::new("data/file2.fna"),
                Path::new("data/file2.txt")
            )
        );
    }

    #[test]
    fn no_files() {
        let fs = file_or_dirfiles(Path::new(""), None);
        assert!(fs.is_err());
    }

    #[test]
    fn only_txt() {
        let mut fs = file_or_dirfiles(Path::new("data"), Some(vec!["txt"])).unwrap();
        fs.sort();

        assert_eq!(
            fs,
            vec!(Path::new("data/file1.txt"), Path::new("data/file2.txt"))
        )
    }
}

// Returns a vector of all files in a given path (file or directory)
// Optionally takes a vector of extensions to match against
pub fn file_or_dirfiles(
    file_or_dir: &Path,
    filter_extensions: Option<Vec<&str>>,
) -> Result<Vec<PathBuf>, Error> {
    let mut all_files: Vec<PathBuf> = Vec::new();

    if file_or_dir.is_file() {
        all_files.push(file_or_dir.to_path_buf());
    } else if file_or_dir.is_dir() {
        all_files = recurse_directory(file_or_dir)?;
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Not a valid file or directory"));
    }

    // If there is a filter on file extensions, apply it here
    if filter_extensions.is_some() {
        // create a HashMap to test existence of extension
        let mut all_filters = HashMap::new();
        for filt in filter_extensions.unwrap() {
            // It is cheaper to convert a &str to OsStr than the reverse
            all_filters.insert(OsStr::new(filt), 1);
        }

        all_files.retain(|x| all_filters.contains_key(x.extension().unwrap()));
    }

    // Check that we have files for use
    if all_files.is_empty(){
        return Err(Error::new(ErrorKind::NotFound, "No valid files found"));
    }

    Ok(all_files)
}

// Path only holds a reference to the path string
// PathBuf owns the string
fn recurse_directory(p: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut af: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(p)? {
        let e = entry?;
        let path = e.path();

        if path.is_dir() {
            recurse_directory(&path)?;
        } else {
            af.push(path);
        }
    }
    Ok(af)
}
