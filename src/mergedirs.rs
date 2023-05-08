use std::path::Path;
use std::fs::{self, File, OpenOptions};
use std::io;

pub enum Mode {
    Copy,
    Append,
    Delete,
}

pub fn merge_dirs(src_dir: &Path, dst_dir: &Path, mode: &Mode) -> io::Result<()> {
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst_dir.join(entry.file_name());

        if src_path.is_file() {
            match mode {
                Mode::Append => {
                    let mut dst_file = OpenOptions::new().write(true).append(true).create(true).open(&dst_path)?;
                    let mut src_file = File::open(src_path)?;
                    io::copy(&mut src_file, &mut dst_file)?;
                }
                Mode::Copy => {
                    fs::copy(&src_path, &dst_path)?;
                }
                Mode::Delete => {
                    fs::remove_file(&dst_path)?;
                }
            }
        } else if src_path.is_dir() {
            merge_dirs(&src_path, &dst_path, mode)?;
        }
    }

    Ok(())
}
