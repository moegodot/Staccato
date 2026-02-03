mod configuration;
mod hooks;
mod platform;

use crate::configuration::Configuration;
use crate::platform::{Architecture, Os, is_os};
use ::color_eyre::eyre;
use ::owo_colors::OwoColorize;
use clap::{Args, Parser, Subcommand};
use color_eyre::eyre::{Context, ContextCompat, eyre};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildingOpts {
    root: PathBuf,
    configuration: Configuration,
}

impl AsRef<Path> for BuildingOpts {
    fn as_ref(&self) -> &Path {
        &self.root
    }
}

impl AsRef<Configuration> for BuildingOpts {
    fn as_ref(&self) -> &Configuration {
        &self.configuration
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, global = true)]
    pub root: Option<String>,

    #[arg(value_enum,short, long, default_value_t = crate::configuration::Configuration::Debug, global=true)]
    pub configuration: Configuration,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Args, Debug)]
struct TargetPlatformArgs {
    target_os: Option<Os>,
    target_architecture: Option<Architecture>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command()]
    UpdateVersion(UpdateVersion),
    #[command()]
    Clean(Clean),
    #[command()]
    BuildRust(BuildRust),
    #[command()]
    BuildManaged(BuildManaged),
    #[command()]
    PreCommit(hooks::PreCommit),
    #[command()]
    PrePush(hooks::PrePush),
}

#[derive(Args, Debug, Clone)]
pub struct BuildGlue {}
impl BuildGlue {
    pub fn invoke(self, _opts: &BuildingOpts) -> eyre::Result<()> {
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct BuildRust {}
impl BuildRust {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        let source = opts.root.as_path();

        let cargo =
            which("cargo".into(), |a, b| a.cmp(b)).wrap_err("failed to find cargo in PATH")?;

        let args = vec![
            "build".into(),
            "--workspace".into(),
            "--profile".into(),
            match opts.configuration {
                Configuration::Debug => "dev".into(),
                Configuration::Release => "release".into(),
            },
        ];

        run(&cargo, &source, args.into_iter(), true, |_| {})?;

        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct UpdateVersion {}
impl UpdateVersion {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        let version_file = opts.root.join(VERSION_FILE_NAME);

        let version = fs::read_to_string(&version_file)?.trim().to_string();
        println!("update version to {}", version.bright_white());

        // write to source/native/rust/Cargo.toml
        {
            let start_mark = "# Version is mantained by sb,do not edit manually - start";
            let end_mark = "# Version is mantained by sb,do not edit manually - end";

            let cargo_toml_file = get_rust_source_dir(opts.root.as_path()).join("Cargo.toml");

            let cargo_toml = fs::read_to_string(&cargo_toml_file)?;

            let start_index = cargo_toml.find(start_mark).unwrap();
            let end_index = cargo_toml.find(end_mark).unwrap();

            let new_cargo_toml = cargo_toml[..(start_index + start_mark.len())].to_string();
            let new_cargo_toml = format!("{}\nversion = \"{}\"\n", new_cargo_toml, version);
            let new_cargo_toml = format!("{}{}", new_cargo_toml, &cargo_toml[end_index..]);

            fs::write(&cargo_toml_file, new_cargo_toml)?;
        }
        println!("update {}", "Cargo.toml".bright_white());
        // write to source/native/managed/Directory.Build.props
        {
            let start_mark = "<!-- Version is mantained by sb,do not edit manually - start -->";
            let end_mark = "<!-- Version is mantained by sb,do not edit manually - end -->";

            let props_file =
                get_managed_source_dir(opts.root.as_path()).join("Directory.Build.props");

            let props = fs::read_to_string(&props_file)?;

            let start_index = props.find(start_mark).unwrap();
            let end_index = props.find(end_mark).unwrap();

            let new_props = props[..(start_index + start_mark.len())].to_string();
            let new_props = format!("{}\n    <Version>{}</Version>\n    ", new_props, version);
            let new_props = format!("{}{}", new_props, &props[end_index..]);

            fs::write(&props_file, new_props)?;
        }
        println!("update {}", "Directory.Build.props".bright_white());

        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct BuildManaged {
    #[arg(short, long, default_value = "net10.0")]
    framework: String,
}
impl BuildManaged {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        let source = get_managed_source_dir(opts.root.as_path());

        let dotnet =
            which("dotnet".into(), |a, b| a.cmp(b)).wrap_err("failed to find dotnet in PATH")?;

        run(
            &dotnet,
            &source,
            vec![
                "build".into(),
                source.to_string_lossy().to_string(),
                "--framework".into(),
                self.framework,
                "--configuration".into(),
                opts.configuration.as_ref().into(),
            ]
            .into_iter(),
            true,
            |_| {},
        )?;

        Ok(())
    }
}

pub fn which<F>(exe: String, sort_func: F) -> Option<std::path::PathBuf>
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
    let mut cmd = std::process::Command::new(&exe);

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

pub fn run_fast(
    exe: &str,
    work_dir: impl AsRef<Path>,
    args: impl Iterator<Item = String>,
    check: bool,
) -> eyre::Result<()> {
    let exe = which(exe.to_string(), |a, b| a.cmp(b)).unwrap_or(PathBuf::from(exe));

    run(exe.as_path(), work_dir.as_ref(), args, check, |_| {})?;

    Ok(())
}

static VERSION_FILE_NAME: &str = "staccato.version";

pub fn get_root_dir() -> eyre::Result<PathBuf> {
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

pub fn get_building_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = root.as_ref().join("building");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_native_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = get_source_dir(root).join("native");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_rust_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = get_native_source_dir(root).join("rust");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_cpp_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = get_native_source_dir(root).join("glue");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_managed_source_dir(root: impl AsRef<Path>) -> PathBuf {
    let r = get_source_dir(root).join("managed");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_opts_dir(opts: &BuildingOpts, name: &str) -> PathBuf {
    let result = opts.root.join(name);

    fs::create_dir_all(&result).unwrap();

    result
}

pub fn get_build_dir(opts: &BuildingOpts) -> PathBuf {
    get_opts_dir(opts, "build")
}

pub fn get_install_dir(opts: &BuildingOpts) -> PathBuf {
    get_opts_dir(opts, "install")
}

pub fn get_output_dir(opts: &BuildingOpts) -> PathBuf {
    get_opts_dir(opts, "dist")
}

#[derive(Args, Debug, Clone)]
pub struct Clean {}
impl Clean {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        let dir = get_build_dir(opts);

        if dir.is_dir() {
            println!("remove build dir: {}", dir.display().bright_white());
            fs::remove_dir_all(&dir)?;
            fs::create_dir_all(&dir)?;
        } else {
            println!(
                "build dir {} not exists, {}",
                "skip".yellow(),
                dir.display().bright_white()
            );
        }
        Ok(())
    }
}

fn real_main() -> eyre::Result<()> {
    let args = Cli::parse();

    let root = if let Some(root) = args.root {
        PathBuf::from(root)
    } else {
        get_root_dir()?
    };

    let opts = BuildingOpts {
        root: root.clone(),
        configuration: args.configuration,
    };

    // execute
    match args.commands {
        Commands::Clean(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::UpdateVersion(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::BuildRust(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::BuildManaged(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::PreCommit(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::PrePush(cmd) => {
            cmd.invoke(&opts)?;
        }
    }

    Ok(())
}

fn main() {
    let now = std::time::Instant::now();

    _ = color_eyre::install()
        .inspect_err(|e| eprintln!("{} to install color-eyre {:?}", "failed".red(), e))
        .unwrap_or(());

    let result = real_main();

    let duration = now.elapsed();

    let code = match result.as_ref() {
        Ok(_) => exit_code::SUCCESS,
        Err(_) => exit_code::FAILURE,
    };

    println!(
        "Elapsed {}{}{} {} == {}",
        duration.as_secs().green(),
        ".".white(),
        duration.subsec_nanos().yellow(),
        "seconds".cyan(),
        humantime::format_duration(duration).to_string().magenta()
    );

    std::process::exit(code);
}
