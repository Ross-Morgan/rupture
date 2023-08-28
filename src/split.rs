use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use crate::utils::{concat_os, file_name_without_extension};
use crate::RuptureError;

pub fn const_split_file<const CHUNK_SIZE: usize>(
    source: PathBuf,
    default_file_name: OsString,
) -> Result<(), RuptureError> {
    let file = match std::fs::File::open(&source) {
        Ok(f) => f,
        Err(err) => return Err(RuptureError::OpenFileError(err)),
    };

    let file_name = file_name_without_extension(source, &default_file_name);

    let mut chunker = ConstChunker::<CHUNK_SIZE>::new(file, file_name);

    match create_local_dir("ruptures".into()) {
        Ok(_) => (),
        Err(err) => return Err(RuptureError::CreateDirError(err)),
    };

    while let Some(name) = chunker.next_chunk() {
        let new_name = concat_os!(OsString::from("./ruptures/"), name);

        let file = match std::fs::File::create(new_name) {
            Ok(f) => f,
            Err(err) => return Err(RuptureError::CreateFileError(err)),
        };

        let mut writer = BufWriter::new(file);

        match writer.write_all(&chunker.buf_contents) {
            Ok(_) => (),
            Err(err) => return Err(RuptureError::WriteFileError(err)),
        };
    }

    Ok(())
}


pub struct ConstChunker<const CHUNK_SIZE: usize> {
    buf_reader: BufReader<File>,
    buf_contents: [u8; CHUNK_SIZE],
    base_name: String,
    counter: usize,
}


impl<const CHUNK_SIZE: usize> ConstChunker<CHUNK_SIZE> {
    fn new(file: File, base_name: OsString) -> Self {
        Self {
            buf_reader: BufReader::with_capacity(CHUNK_SIZE, file),
            buf_contents: [0u8; CHUNK_SIZE],
            base_name: base_name.to_str().unwrap().to_string(),
            counter: 0,
        }
    }

    fn next_chunk(&mut self) -> Option<OsString> {
        self.buf_reader
            .read_exact(&mut self.buf_contents)
            .ok()
            .map(|_| self.generate_next_name())
    }

    fn increment(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    fn generate_next_name(&mut self) -> OsString {
        let count = self.increment();
        
        concat_os!(
            OsString::from(self.base_name.as_str()),
            OsString::from("-"),
            OsString::from(count.to_string()),
            OsString::from(".rupture")
        )
    }
}


fn create_local_dir(name: String) -> Result<(), std::io::Error> {
    std::fs::create_dir(name)
}
