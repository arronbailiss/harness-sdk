use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn collect_rust_files(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();

        if file_type.is_dir() {
            collect_rust_files(&path, files)?;
        } else if file_type.is_file() && path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let manifest_directory = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "CARGO_MANIFEST_DIR is not set"))?,
    );
    let docs_directory = manifest_directory.join("../src/content/docs");
    let output_directory = PathBuf::from(
        env::var_os("OUT_DIR").ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "OUT_DIR is not set"))?,
    );

    let mut files = Vec::new();
    collect_rust_files(&docs_directory, &mut files)?;
    files.sort();

    println!("cargo:rerun-if-changed={}", docs_directory.display());

    let mut modules = File::create(output_directory.join("snippets.rs"))?;
    for (index, path) in files.iter().enumerate() {
        println!("cargo:rerun-if-changed={}", path.display());
        let canonical_path = path.canonicalize()?;
        writeln!(
            modules,
            "#[path = {:?}]\nmod snippet_{index};",
            canonical_path.to_string_lossy()
        )?;
    }

    Ok(())
}
