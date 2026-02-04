use std::{
    env, fs,
    path::{Path, PathBuf},
};

use eyre::Context;

use crate::BuildingOpts;

pub static VERSION_FILE_NAME: &str = "staccato.version";

pub static STACCATO_PROJECT_FILE_NAME: &str = "staccato.project.toml";

pub fn get_user_project_root(
    root: &Path,
    building_internal_samples: &mut bool,
) -> eyre::Result<PathBuf> {
    let mut current_dir = root.to_path_buf();

    let version_file = current_dir.join(STACCATO_PROJECT_FILE_NAME);

    if version_file.is_file() {
        return Ok(current_dir);
    }

    while let Some(parent) = current_dir.parent() {
        let version_file = parent.join(STACCATO_PROJECT_FILE_NAME);

        if version_file.is_file() {
            return Ok(parent.to_path_buf());
        }

        current_dir = parent.to_path_buf();
    }

    // found no user's project file,use `staccato/sample`
    *building_internal_samples = true;
    Ok(root.join("sample"))
}

pub fn get_staccato_root_dir() -> eyre::Result<PathBuf> {
    let mut current_dir = env::current_dir().wrap_err("failed to get current dir")?;

    let version_file = current_dir.join(VERSION_FILE_NAME);

    if version_file.is_file() {
        return Ok(current_dir);
    }

    while let Some(parent) = current_dir.parent() {
        let version_file = parent.join(VERSION_FILE_NAME);

        if version_file.is_file() {
            return Ok(parent.to_path_buf());
        }

        current_dir = parent.to_path_buf();
    }

    Err(eyre::eyre!(
        "failed to find root dir, no `{}` file found in current or any parent dirs",
        VERSION_FILE_NAME
    ))
}

pub fn get_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = root.as_ref().join("source");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn _get_xtask_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = root.as_ref().join("building");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn _get_native_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = get_source_dir(root).join("native");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn _get_rust_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = _get_native_source_dir(root).join("rust");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn _get_glue_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = _get_native_source_dir(root).join("glue");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_managed_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = get_source_dir(root).join("managed");
    fs::create_dir_all(&r).unwrap();
    r
}

fn get_opts_dir(opts: &BuildingOpts, name: &str) -> PathBuf {
    let result = opts.staccato_root.join(name);

    fs::create_dir_all(&result).unwrap();

    result
}

pub fn get_build_dir(opts: &BuildingOpts) -> PathBuf {
    get_opts_dir(opts, "build")
}

pub fn _get_output_dir(opts: &BuildingOpts) -> PathBuf {
    get_opts_dir(opts, "dist")
}
