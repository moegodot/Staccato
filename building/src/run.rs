use eyre::eyre;
use owo_colors::OwoColorize;
use std::path::{Path, PathBuf};

use crate::{
    BuildingOpts,
    paths::get_managed_source_dir,
    platform::{Os, is_os},
};

pub fn which(exe: impl AsRef<str>) -> PathBuf {
    let exe = exe.as_ref().to_string();
    which_cmp(exe.clone(), |a, b| a.cmp(b)).unwrap_or(PathBuf::from(exe))
}

pub fn which_cmp<F>(exe: String, sort_func: F) -> Option<std::path::PathBuf>
where
    F: FnMut(&std::path::PathBuf, &std::path::PathBuf) -> std::cmp::Ordering,
{
    let path = std::env::var_os("PATH")?;
    let path_ext = if is_os(Os::Windows) {
        let path_ext = std::env::var("PATHEXT").ok()?;

        let path_ext: Vec<String> = path_ext.split(";").map(|s| s.into()).collect();

        vec!["".into()]
            .into_iter()
            .chain(path_ext.clone().into_iter().map(|s| s.to_ascii_uppercase()))
            .chain(path_ext.into_iter().map(|s| s.to_ascii_uppercase()))
            .collect()
    } else {
        vec!["".into()]
    };

    let mut found = vec![];

    for path in std::env::split_paths(&path) {
        for ext in &path_ext {
            let candidate = path.join(format!("{}{}", exe, ext));

            if candidate.is_file() {
                found.push(candidate);
            }
        }
    }

    found.sort_by(sort_func);

    found.reverse();

    found.into_iter().next()
}

pub fn run<F>(
    exe: &Path,
    work_dir: &Path,
    args: impl Iterator<Item = String>,
    check: bool,
    f: F,
) -> eyre::Result<()>
where
    F: FnOnce(&mut std::process::Command),
{
    let mut cmd = std::process::Command::new(exe);

    cmd.current_dir(work_dir);

    let collected: Vec<String> = args.collect();

    cmd.args(collected.iter());

    f(&mut cmd);

    println!(
        "{}",
        format!(
            "spawn in {:?}: {:?} {}",
            work_dir.bright_white(),
            exe.bright_cyan(),
            collected.join(" ").bright_blue()
        )
        .underline()
    );

    let mut process = cmd.spawn()?;

    let exit = process.wait()?;

    let code = exit
        .code()
        .ok_or(eyre!("failed to get exit code of the process"))?;

    let success = code == 0;

    println!(
        "process exit, code {}",
        if success {
            code.green().to_string()
        } else {
            code.red().to_string()
        }
    );

    if (!success) && check {
        return Err(eyre::eyre!(
            "the process failed with code {}, and argument `check` is true",
            code
        ));
    }

    Ok(())
}

pub fn run_command(exe: &Path, work_dir: &Path, args: &[&str]) -> eyre::Result<()> {
    run(
        exe,
        work_dir,
        args.iter().map(|arg| (*arg).to_string()),
        true,
        |_| {},
    )
}

pub fn run_cargo(opts: &BuildingOpts, args: &[&str]) -> eyre::Result<()> {
    let cargo = which("cargo");
    run_command(&cargo, opts.staccato_root.as_path(), args)
}

pub fn run_dotnet(opts: &BuildingOpts, args: &[&str]) -> eyre::Result<()> {
    let dotnet = which("dotnet");
    let source = get_managed_source_dir(opts.staccato_root.as_path());
    run_command(&dotnet, &source, args)
}
