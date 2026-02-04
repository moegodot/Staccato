mod actions;
mod configuration;
mod hooks;
mod paths;
mod platform;
mod run;

use crate::configuration::Configuration;
use crate::paths::{
    VERSION_FILE_NAME, get_build_dir, get_managed_source_dir, get_staccato_root_dir,
    get_user_project_root,
};
use crate::platform::{Architecture, Os};
use crate::run::which;
use ::color_eyre::eyre;
use ::owo_colors::OwoColorize;
use clap::{Args, Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildingOpts {
    /// are we building sample projects of staccato self now?
    building_internal_samples: bool,
    /// user's project root path,it maybe staccato's sample also
    user_project_root: PathBuf,
    /// the staccato root path
    staccato_root: PathBuf,
    /// build configuration
    configuration: Configuration,
}

impl AsRef<Path> for BuildingOpts {
    fn as_ref(&self) -> &Path {
        &self.staccato_root
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
    pub staccato_root: Option<String>,

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
    #[command()]
    Test(Test),
    #[command()]
    Lint(Lint),
    #[command()]
    Format(Format),
}

#[derive(Args, Debug, Clone)]
pub struct Format {}
impl Format {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        actions::format(opts)?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct Test {}
impl Test {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        actions::run_tests(opts)?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct Lint {}
impl Lint {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        actions::run_lint(opts)?;
        Ok(())
    }
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
        let source = opts.staccato_root.as_path();

        let cargo = which("cargo");

        let args = vec![
            "build".into(),
            "--workspace".into(),
            "--profile".into(),
            match opts.configuration {
                Configuration::Debug => "dev".into(),
                Configuration::Release => "release".into(),
            },
        ];

        run::run(&cargo, source, args.into_iter(), true, |_| {})?;

        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct UpdateVersion {
    #[arg(long, default_value_t = false)]
    verify_no_changes: bool,
}
impl UpdateVersion {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        let version_file = opts.staccato_root.join(VERSION_FILE_NAME);

        let version = fs::read_to_string(&version_file)?.trim().to_string();
        println!("update version to {}", version.bright_white());

        let mut no_change = true;

        // write to Cargo.toml
        println!("updating {}", "Cargo.toml".bright_white());
        {
            let start_mark = "# Version is mantained by sb,do not edit manually - start";
            let end_mark = "# Version is mantained by sb,do not edit manually - end";

            let cargo_toml_file = opts.staccato_root.as_path().join("Cargo.toml");

            let cargo_toml = fs::read_to_string(&cargo_toml_file)?;
            let old_cargo_toml = cargo_toml.clone();

            let start_index = cargo_toml.find(start_mark).unwrap();
            let end_index = cargo_toml.find(end_mark).unwrap();

            let new_cargo_toml = cargo_toml[..(start_index + start_mark.len())].to_string();
            let new_cargo_toml = format!("{}\nversion = \"{}\"\n", new_cargo_toml, version);
            let new_cargo_toml = format!("{}{}", new_cargo_toml, &cargo_toml[end_index..]);

            if self.verify_no_changes {
                if old_cargo_toml == new_cargo_toml {
                    println!("the `version` of Cargo.toml is already up to date");
                } else {
                    no_change = false;
                    println!("the `version` of Cargo.toml is out of date");
                }
            } else {
                fs::write(&cargo_toml_file, new_cargo_toml)?;
            }
        }
        // write to source/native/managed/Directory.Build.props
        println!("updating {}", "Directory.Build.props".bright_white());
        {
            let start_mark = "<!-- Version is mantained by sb,do not edit manually - start -->";
            let end_mark = "<!-- Version is mantained by sb,do not edit manually - end -->";

            let props_file =
                get_managed_source_dir(opts.staccato_root.as_path()).join("Directory.Build.props");

            let props = fs::read_to_string(&props_file)?;
            let old_props = props.clone();

            let start_index = props.find(start_mark).unwrap();
            let end_index = props.find(end_mark).unwrap();

            let new_props = props[..(start_index + start_mark.len())].to_string();
            let new_props = format!("{}\n    <Version>{}</Version>\n    ", new_props, version);
            let new_props = format!("{}{}", new_props, &props[end_index..]);

            if self.verify_no_changes {
                if old_props == new_props {
                    println!("the `version` of Directory.Build.props is already up to date");
                } else {
                    no_change = false;
                    println!("the `version` of Directory.Build.props is out of date");
                }
            } else {
                fs::write(&props_file, new_props)?;
            }
        }

        if self.verify_no_changes && (!no_change) {
            eyre::bail!("version is out of date");
        }

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
        let source = get_managed_source_dir(opts.staccato_root.as_path());

        let dotnet = which("dotnet");

        run::run(
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

    let root = if let Some(root) = args.staccato_root {
        PathBuf::from(root)
    } else {
        get_staccato_root_dir()?
    };

    let mut building_internal_samples = false;

    let opts = BuildingOpts {
        user_project_root: args
            .root
            .map(PathBuf::from)
            .unwrap_or(get_user_project_root(
                root.as_path(),
                &mut building_internal_samples,
            )?),
        staccato_root: root.clone(),
        configuration: args.configuration,
        building_internal_samples,
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
        Commands::Test(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::Format(cmd) => {
            cmd.invoke(&opts)?;
        }
        Commands::Lint(cmd) => {
            cmd.invoke(&opts)?;
        }
    }

    Ok(())
}

fn main() {
    let now = std::time::Instant::now();

    _ = color_eyre::install()
        .inspect_err(|e| eprintln!("{} to install color-eyre {:?}", "failed".red(), e));

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
