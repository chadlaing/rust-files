use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_files() {
        let mut fs = file_or_dirfiles(Path::new("data/"), None).unwrap();
        fs.sort();

        assert_eq!(fs, vec!(
            Path::new("data/file1.fasta"),
            Path::new("data/file1.txt"),
            Path::new("data/file2.fna"),
            Path::new("data/file2.txt")
        ));
    }

    #[test]
    fn no_files(){
        let fs = file_or_dirfiles(Path::new(""),None);
        assert!(fs.is_err());
    }
}



/// Returns a vector of all files in a given path (file or directory)
/// Optionally takes a vector of extensions to match agains
/// # Examples
///
/// # /dir/fasta1.fasta
/// # /dir/fasta2.fasta
/// # /dir/not_fasta.txt
///
/// ```
///

/// ```
///
pub fn file_or_dirfiles(file_or_dir: &Path, file_types: Option<Vec<&str>>) -> Result<Vec<PathBuf>, Error> {
    if file_or_dir.is_file() {

        // If there was a supplied vec of extensions, match against them
        Ok(vec![file_or_dir.to_path_buf()])
    } else if file_or_dir.is_dir() {
        let all_files = recurse_directory(file_or_dir)?;
        Ok(all_files)
    } else {
        Err(Error::new(ErrorKind::NotFound, "No valid files found"))
    }
}
        // Of all the files, we want to keep only the fasta files
//        let mut subset_files: Vec<PathBuf> = Vec::new();

        // To ensure we only get fasta files, we open each file, and attempt
        // to get the first fasta record of each. If we succeed, we add the file to
        // the vector of fasta files. If not, we do nothing.
//        for f in all_files {
//
//        }

        // Check to see if any fasta files were found. If not, return an error
//        if subset_files.is_empty() {
//            Err(Error::new(
//                ErrorKind::NotFound,
//                "No valid files of type found",
//            ))
//        } else {
//            Ok(subset_files)
//        }



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
