use std::{fs::File, io::{BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}};

use crate::{array_from_slice, bytes_to_num, error::RuptureError};

pub fn juncture_file(dir: PathBuf, extension: impl Into<String>, rupture_block_size: u64, assume_all_valid: bool) -> crate::Result<()> {
    assert!(dir.is_dir());

    let rupture_out_file_name = rupture_dir_to_file_name(dir.clone(), extension.into());
    let rupture_dir = dir.into_os_string().into_string().expect("Invalid unicode bytes");

    let mut paths = glob::glob(&format!("{rupture_dir}/*.{}", crate::RUPTURE_EXTENSION))
        .expect("Couldn't read dir")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    paths.sort();

    if !assume_all_valid {
        todo!();
    }

    let mut file_readers = paths
        .into_iter()
        .map(File::open)
        .map(|f| f.expect("Failed to open rupture file"))
        .map(|f| BufReader::with_capacity(rupture_block_size as usize, f))
        .collect::<Vec<_>>();

    let mut buffers = vec![[0u8; {std::mem::size_of::<u64>() * 2}]; file_readers.len()];

    let results =  file_readers
        .iter_mut()
        .zip(buffers.as_mut_slice())
        .map(|(reader, buf)| reader.read_exact(buf))
        .collect::<Vec<_>>();

    for r in results {
        r.map_err(RuptureError::ReadFileError)?;
    }

    let mut size_and_totals = buffers
        .into_iter()
        .map(|a| {
            let (lhs, rhs) = a.split_at(8);
            (array_from_slice(lhs), array_from_slice::<_, 8>(rhs))
        })
        .map(|(lhs, rhs)| (bytes_to_num(lhs), bytes_to_num(rhs)))
        .enumerate()
        .collect::<Vec<_>>();

    let first_size_total = size_and_totals[0].1;

    if !size_and_totals
        .iter()
        .skip(1)
        .all(|&(_, s)| s.0 != first_size_total.0 && s.1 == first_size_total.1)
    {
        return Err(RuptureError::RuptureFileError);
    }

    size_and_totals.sort_by(|(_, a), (_, b)| (a.0).cmp(&b.0));

    let indices = size_and_totals
        .iter()
        .map(|&(i, _)| i);

    let out_file = File::create(rupture_out_file_name).map_err(RuptureError::CreateFileError)?;
    let mut writer = BufWriter::with_capacity(rupture_block_size as usize, out_file);

    let mut rupture_block_buffer = Vec::<u8>::with_capacity(rupture_block_size as usize);

    for idx in indices {
        let reader = &mut file_readers[idx];
        
        reader.read_exact(rupture_block_buffer.as_mut_slice()).map_err(RuptureError::ReadFileError)?;
        writer.write_all(&rupture_block_buffer).map_err(RuptureError::WriteFileError)?;
    }

    Ok(())
}

fn rupture_dir_to_file_name(p: PathBuf, extension: String) -> PathBuf {
    let file_name = p.file_stem().unwrap_or(std::ffi::OsStr::new("rupture-out")).to_str().unwrap_or("rupture-out");
    let base_dir = p.ancestors().skip(1).next().unwrap_or(&Path::new(".")).to_path_buf();
    base_dir.join(format!("{file_name}.{extension}"))
}
