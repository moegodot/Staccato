mod platform;
mod configuration;

use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use clap::{Args, Parser, Subcommand};
use ::color_eyre::eyre;
use ::owo_colors::OwoColorize;
use color_eyre::eyre::{eyre, Context, ContextCompat};
use ignore::WalkBuilder;
use crate::configuration::Configuration;
use crate::platform::{is_os, Architecture, Os, Platform};

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuildingOpts{
    root: PathBuf,
    platform: Platform,
    configuration: Configuration,
}

impl AsRef<Path> for BuildingOpts{
    fn as_ref(&self) -> &Path {
        &self.root
    }
}

impl AsRef<Platform> for BuildingOpts{
    fn as_ref(&self) -> &Platform {
        &self.platform
    }
}

impl AsRef<Configuration> for BuildingOpts{
    fn as_ref(&self) -> &Configuration {
        &self.configuration
    }
}

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long,global=true)]
    pub root: Option<String>,

    #[arg(short, long, default_value_t = true)]
    pub clean: bool,

    #[arg(short, long, default_value_t = true)]
    pub upgrade:bool,

    #[arg(value_enum,short, long, default_value_t = crate::configuration::Configuration::Debug, global=true)]
    pub configuration:Configuration,

    #[command(flatten)]
    pub target:TargetPlatformArgs,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Args, Debug)]
struct TargetPlatformArgs{
    target_os: Option<Os>,
    target_architecture: Option<Architecture>,
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    #[command()]
    BuildGlue(BuildGlue),
    #[command()]
    BuildRust(BuildRust),
    #[command()]
    BuildManaged(BuildManaged),
    #[command(visible_alias = "build")]
    BuildAll{
        #[command(flatten)]
        glue: BuildGlue,
        #[command(flatten)]
        rust: BuildRust,
        #[command(flatten)]
        managed: BuildManaged,
    },
}

#[derive(Args,Debug,Clone)]
pub struct BuildGlue{}
impl BuildGlue{
    pub fn invoke(self,opts:&BuildingOpts) -> eyre::Result<()> {
        Ok(())
    }
}

#[derive(Args,Debug,Clone)]
pub struct BuildRust{}
impl BuildRust{
    pub fn invoke(self,opts:&BuildingOpts) -> eyre::Result<()> {
        let source = get_rust_source_dir(opts.root.as_path());

        let cargo = which("cargo".into(), |a,b| a.cmp(b))
            .wrap_err("failed to find cargo in PATH")?;

        let mut args = vec![
            "build".into(),
            "--workspace".into(),
            "--profile".into(),
            match opts.configuration{
                Configuration::Debug => "dev".into(),
                Configuration::Release => "release".into(),
            },
            "--target-dir".into(),
            get_build_dir(&opts).to_string_lossy().to_string()
        ];

        if let Some(target) = opts.platform.rust_target_triple() {
            args.push("--target".into());
            args.push(target.into());
        }

        run(&cargo, &source, args.into_iter(), true,|_|{})?;

        Ok(())
    }
}

#[derive(Args,Debug,Clone)]
pub struct BuildManaged{
    #[arg(short, long, default_value = "net10.0")]
    framework:String,
}
impl BuildManaged{
    pub fn invoke(self,opts:&BuildingOpts) -> eyre::Result<()> {
        let source = get_managed_source_dir(opts.root.as_path());

        let dotnet = which("dotnet".into(), |a,b| a.cmp(b))
            .wrap_err("failed to find dotnet in PATH")?;

        run(&dotnet, &source, vec![
            "build".into(),
            source.to_string_lossy().to_string(),
            /*
            enable when AOT,but we needn't now
            "--runtime".into(),
            format!("{}-{}",
            match opts.platform.os.unwrap_or_default(){
                Os::Windows => "win",
                Os::Linux => "linux",
                Os::MacOS => "osx",
            },
            match opts.platform.architecture.unwrap_or_default(){
                Architecture::X64 => "x64",
                Architecture::Arm64 => "arm64",
            }),
            */
            "--framework".into(),
            self.framework,
            "--configuration".into(),
            opts.configuration.as_ref().into()].into_iter(),true,|_|{})?;

        Ok(())
    }
}

fn copy_sb_to_old(exe_path:&Path) -> eyre::Result<()> {
    if cfg!(target_os = "windows") && exe_path.exists() {
        let mut old_path = exe_path.to_path_buf();
        old_path.set_extension("exe.old");
        let _ = fs::remove_file(&old_path);
        println!("move {} to {}", exe_path.display().bright_white(), old_path.display().bright_white());
        fs::rename(exe_path, old_path)?;
    }

    Ok(())
}

/// 检查源目录是否比目标二进制文件更新
///
/// # 参数
/// * `source_dir` - 源码根目录（例如 ./building）
/// * `binary_path` - 编译出的二进制路径（例如 ./target/release/sb）
fn is_source_updated(source_dir: &Path, binary_path: &Path) -> bool {

    // 1. 如果二进制文件不存在，必然需要更新
    let binary_metadata = match fs::metadata(binary_path) {
        Ok(m) => m,
        Err(_) => return true,
    };

    let binary_mtime = binary_metadata
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH);

    // 2. 使用 ignore crate 遍历源码目录
    // 它会自动处理 .gitignore 和隐藏文件
    let walker = WalkBuilder::new(source_dir)
        .standard_filters(true) // 启用 .gitignore, .ignore 等过滤
        .hidden(true)           // 忽略隐藏文件
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                // 只检查文件
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(mtime) = metadata.modified() {
                            // 如果有任何一个文件的修改时间晚于二进制文件
                            if mtime > binary_mtime {
                                // 打印一下哪个文件变了，方便调试
                                println!("trigger sb upgrade by: {:?}", entry.path());
                                return true;
                            }
                        }
                    }
                }
            }
            Err(err) => eprintln!("get error when check staccato-build source mtime: {}", err),
        }
    }

    // 遍历结束没发现更新的文件
    false
}

pub fn which<F>(exe:String, sort_func:F) -> Option<std::path::PathBuf>
where F: FnMut(&std::path::PathBuf,&std::path::PathBuf) -> std::cmp::Ordering {
    let path = std::env::var_os("PATH")?;
    let path_ext = if is_os(Os::Windows){
        let path_ext = std::env::var("PATHEXT").ok()?;

        let path_ext:Vec<String> = path_ext.split(";").map(|s| s.into()).collect();

        vec!["".into()].into_iter()
            .chain(path_ext.clone().into_iter().map(|s| s.to_ascii_uppercase()))
            .chain(path_ext.into_iter().map(|s| s.to_ascii_uppercase())).collect()
    }
    else{
        vec!["".into()]
    };

    let mut found = vec![];

    for path in std::env::split_paths(&path){
        for ext in &path_ext{
            let candidate = path.join(format!("{}{}",exe,ext));

            if candidate.is_file(){
                found.push(candidate);
            }
        }
    }

    found.sort_by(sort_func);

    found.reverse();

    found.into_iter().next()
}

pub fn run<F>(exe:&Path, work_dir:&Path, args:impl Iterator<Item = String>, check:bool, f: F) -> eyre::Result<()>
    where F:FnOnce(&mut std::process::Command){

    let mut cmd = std::process::Command::new(&exe);

    cmd.current_dir(work_dir);

    let collected:Vec<String> = args.collect();

    cmd.args(collected.iter());

    f(&mut cmd);

    println!("{}",
    format!("spawn in {:?}: {:?} {}", work_dir.bright_white(), exe.bright_cyan(), collected.join(" ").bright_blue()).underline());

    let mut process = cmd.spawn()?;

    let exit = process.wait()?;

    let code = exit.code().ok_or(eyre!("failed to get exit code of the process"))?;

    let success = code == 0;

    println!("process exit, code {}", if success { code.green().to_string() } else { code.red().to_string() });

    if (!success) && check{
        return Err(eyre::eyre!("the process failed with code {}, and argument `check` is true",code));
    }

    Ok(())
}

static VERSION_FILE_NAME: &str = "staccato.version";

pub fn get_root_dir() -> eyre::Result<PathBuf>{
    let mut current_dir = env::current_dir().wrap_err("failed to get current dir")?;

    let version_file = current_dir.join(VERSION_FILE_NAME);

    if version_file.is_file(){
        return Ok(current_dir);
    }

    while let Some(parent) = current_dir.parent(){
        let version_file = parent.join(VERSION_FILE_NAME);

        if version_file.is_file(){
            return Ok(parent.to_path_buf());
        }

        current_dir = parent.to_path_buf();
    }

    Err(eyre::eyre!("failed to find root dir, no `{}` file found in current or any parent dirs",VERSION_FILE_NAME))
}

pub fn get_source_dir(root: impl AsRef<Path>) -> PathBuf{
    let r= root.as_ref().join("source");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_building_dir(root: impl AsRef<Path>) -> PathBuf{
    let r= root.as_ref().join("building");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_native_source_dir(root: impl AsRef<Path>) -> PathBuf{
    let r= get_source_dir(root).join("native");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_rust_source_dir(root: impl AsRef<Path>) -> PathBuf{
    let r= get_native_source_dir(root).join("rust");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_cpp_source_dir(root: impl AsRef<Path>) -> PathBuf{
    let r= get_native_source_dir(root).join("glue");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_managed_source_dir(root: impl AsRef<Path>) -> PathBuf{
    let r= get_source_dir(root).join("managed");
    fs::create_dir_all(&r).unwrap();
    r
}

pub fn get_opts_dir(opts:&BuildingOpts, name:&str) -> PathBuf{
    let architecture = opts.platform.architecture.unwrap_or_default();
    let architecture = architecture.as_ref();
    let os = opts.platform.os.unwrap_or_default();
    let os = os.as_ref();
    let configuration = opts.configuration.as_ref();

    let result = opts.root.join(name).join(architecture).join(os).join(configuration);

    fs::create_dir_all(&result).unwrap();

    result
}

pub fn get_build_dir(opts:&BuildingOpts) -> PathBuf{
    get_opts_dir(opts,"build")
}

pub fn get_install_dir(opts:&BuildingOpts) -> PathBuf{
    get_opts_dir(opts,"install")
}

pub fn get_artifact_dir(opts:&BuildingOpts) -> PathBuf{
    get_opts_dir(opts,"artifact")
}

pub fn install_sb(source:&Path) -> eyre::Result<()>{
    let cargo = which("cargo".into(), |a,b| a.cmp(b))
        .wrap_err("failed to find cargo in PATH")?;

    run(&cargo, source, vec!["install".into(),"--path".into(),source.to_string_lossy().to_string()].into_iter(), true,|_|{})?;

    Ok(())
}

pub fn run_sb() -> eyre::Result<()>{
    let sb_path = which("sb".into(), |a,b| {
        if let (Some(a_meta),Some(b_meta)) = (a.metadata().ok(),b.metadata().ok()){
            if let (Ok(a_time),Ok(b_time)) = (a_meta.modified(),b_meta.modified()){
                return a_time.cmp(&b_time);
            }
        }
        a.cmp(b)
    })
        .wrap_err("failed to find sb in PATH")?;

    let current_dir = env::current_dir()
        .wrap_err("failed to get current dir")?;

    run(&sb_path, &current_dir, env::args().into_iter().skip(1), true, |_|{})?;

    Ok(())
}

pub fn upgrade(root:&Path,binary:PathBuf) -> eyre::Result<()> {
    let building_dir = get_building_dir(get_root_dir()?);

    if !is_source_updated(&building_dir, &binary) {
        println!("staccato-build source not updated, {} upgrade", "skip".green());
        Ok(())
    } else {
        println!("staccato-build source updated, {}...", "upgrading".blue());

        copy_sb_to_old(&binary)?;

        install_sb(&building_dir)?;

        // run again

        if let Err(err) = run_sb(){
            println!("failed to run sb: {}", err);
            std::process::exit(1);
        }
        std::process::exit(0);
    }
}

pub fn clean(opts:&BuildingOpts) -> eyre::Result<()> {
    let dir = get_build_dir(opts);

    if dir.is_dir(){
        println!("remove build dir: {}",dir.display().bright_white());
        fs::remove_dir_all(&dir)?;
        fs::create_dir_all(&dir)?;
    }
    else{
        println!("build dir {} not exists, {}","skip".yellow(),dir.display().bright_white());
    }

    Ok(())
}

fn real_main() -> eyre::Result<()> {
    let mut updated = false;
    if let Ok(root) = get_root_dir() && let Ok(binary) = env::current_exe(){
        upgrade(&root,binary)?; // 先手升级
        updated = true;
    }

    let args = Cli::parse();

    let root = if let Some(root) = args.root{
        PathBuf::from(root)
    }
    else{
        get_root_dir()?
    };

    if !updated {
        let binary = env::current_exe();

        if let Ok(binary) = binary {
            upgrade(&root, binary)?;
        } else {
            println!("{} to get {}, {} upgrade check", "failed".red(), "current exe path".cyan(), "skip".yellow());
        }
    }

    let opts = BuildingOpts{
        root: root.clone(),
        platform: Platform{
            os: args.target.target_os,
            architecture: args.target.target_architecture,
        },
        configuration: args.configuration,
    };

    // execute
    match args.commands{
        Commands::BuildGlue(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::BuildRust(cmd) => {
            cmd.invoke(&opts)?;
        },
        Commands::BuildManaged(cmd) => {
            cmd.invoke(&opts)?;
        },
        Commands::BuildAll{
            glue,
            rust,
            managed,
        } => {
            BuildGlue::invoke(glue,&opts)?;
            BuildRust::invoke(rust,&opts)?;
            BuildManaged::invoke(managed,&opts)?;
        }
    }

    if args.clean{
        clean(&opts)?;
    }

    Ok(())
}

fn main() {
    let now = std::time::Instant::now();

    _ = color_eyre::install()
        .inspect_err(|e| eprintln!("{} to install color-eyre {:?}","failed".red(), e))
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
