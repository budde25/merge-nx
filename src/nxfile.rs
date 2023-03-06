use crate::error::Error;
use std::io::Write;

use camino::Utf8PathBuf;
use indicatif::{ProgressBar, ProgressStyle};

use crate::filetype::FileType;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct NxFile {
    directory: Utf8PathBuf,
    name: String,
    file_type: FileType,
}

impl NxFile {
    pub fn from_path_buf(path: Utf8PathBuf) -> Result<Self, Error> {
        if !path.is_file() {
            return Err(Error::FileDoesNotExist(path));
        }

        let name = path
            .file_stem()
            .expect("Path should have stem as it exists")
            .to_owned();

        let file_type = path.extension().unwrap_or_default().parse()?;
        let mut directory = path;
        directory.pop();

        Ok(Self {
            directory,
            name,
            file_type,
        })
    }

    fn files(&self) -> glob::Paths {
        let mut glob_pattern = self.directory.clone();
        glob_pattern.push(&self.name);
        let mut glob_str = glob_pattern.to_string();
        glob_str.push('.');
        glob_str.push_str(self.file_type.glob_pattern());

        glob::glob(&glob_str).expect("Pattern is valid unix")
    }

    fn default_ouput(&self) -> Utf8PathBuf {
        let mut directory = self.directory.clone();
        directory.push(&self.name);
        directory.set_extension(self.file_type.extension());
        directory
    }

    pub fn merge(self, output: Option<Utf8PathBuf>, delete: bool) -> anyhow::Result<()> {
        let mut output = output.unwrap_or_else(|| self.default_ouput());
        // add a .part so its it is in progess
        let org_output = output.clone();
        output.set_extension(format!("{}.part", output.extension().unwrap_or_default()));

        let mut output_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&output)?;

        let input_files: Vec<_> = self.files().flatten().collect();
        let total_size: u64 = input_files
            .iter()
            .map(|x| std::fs::metadata(x).unwrap().len())
            .sum();

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
        .unwrap()
        .progress_chars("#>-"));

        for file in input_files.iter() {
            let Ok(mut input_file) = std::fs::File::open(file) else {
                continue;
            };

            let chunk_size = 0x4000; // 4 KiB

            loop {
                use std::io::Read;
                let mut chunk = Vec::with_capacity(chunk_size);

                let n = std::io::Read::by_ref(&mut input_file)
                    .take(chunk_size as u64)
                    .read_to_end(&mut chunk)
                    .unwrap();

                pb.inc(n as u64);

                if n == 0 {
                    break;
                }

                output_file.write_all(&chunk).unwrap();

                if n < chunk_size {
                    break;
                }
            }
        }

        std::fs::rename(&output, &org_output)?;

        pb.finish();
        println!("Merged files as '{}'!", org_output);

        if delete {
            for file in input_files.iter() {
                std::fs::remove_file(file)?;
            }
        }

        Ok(())
    }
}
