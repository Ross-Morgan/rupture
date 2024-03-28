use rupture::prelude::*;

fn main() -> rupture::Result<()> {
    rupture_file("./big-boi.bin".into(), 2u64.pow(18), 2u64.pow(16))?;

    std::thread::sleep(std::time::Duration::from_secs(4));
    
    juncture_file("./big-boi-ruptures/".into(), "bin", 2u64.pow(18), true)?;

    Ok(())
}