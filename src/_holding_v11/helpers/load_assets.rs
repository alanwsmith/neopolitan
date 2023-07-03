use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

pub fn load_assets(source_dir: &str, dest_dir: &str) -> Result<u64, fs_extra::error::Error> {
    let mut options = CopyOptions::new();
    options.content_only = true;
    options.overwrite = true;
    copy(source_dir, dest_dir, &options)?;
    Ok(0)
}
