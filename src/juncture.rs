pub fn juncture_file(dir: PathBuf, rupture_block_size: u64) -> crate::Result<()> {
    assert!(dir.is_dir());

    let rupture_dir = dir.into_os_string().into_string().expect("Invalid unicode bytes");

    let

    let paths = glob::glob(&format!("{rupture_dir}/*.{}", crate::RUPTURE_EXTENSION))
        .expect("Couldn't read dir")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for path in paths.iter() {
        
    }