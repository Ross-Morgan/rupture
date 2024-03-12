use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn rupture_bytes(source: PathBuf, rupture_size: u64, rupture_block_size: u64) -> crate::Result<()> {
    let file_name = match source.file_stem() {
        Ok(oss) => oss.to_string(),
        None => String::new(),
    };
    
    let file = File::new(source.clone());
    
    let file_size = file.metadata().unwrap().len();

    let rupture_dir_name = format!("{file_name}-ruptures").trim_start_matches('-');
    let rupture_file_name = rupture_dir_name.trim_end_matches('s');

    std::io::create_dir(rupture_dir_name)?;
    
    let rupture_count = file_size.div_ceil(rupture_size);
    let rupture_block_count = rupture_size.div_ceil(rupture_block_size);
    
    let mut reader = BufReader::with_capacity(rupture_block_size, file);
    let mut rupture_block_buffer = Vec::<u8>::with_capacity(rupture_block_size);

    for file_number in 0..rupture_count {
        let rupture_file = File::create([".", &rupture_dir_name, format!("{rupture_file_name}-{file_number}.rupt")].iter().collect::<PathBuf>()).unwrap();
        let mut writer = BufWriter::with_capacity(rupture_block_size, rupture_file);

        for _ in 0..rupture_block_count {
            let block_bytes = reader.read_exact(rupture_block_buffer.as_mut_slice()).unwrap();
            write.write_all(&rupture_block_buffer).unwrap();
        }
    }

    Ok(())
}
