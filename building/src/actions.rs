use crate::{
    BuildingOpts,
    run::{run_cargo, run_dotnet},
};

pub fn check_well_format(opts: &BuildingOpts) -> eyre::Result<()> {
    run_dotnet(opts, &["format", "--verify-no-changes"])?;
    run_cargo(opts, &["xtask", "update-version", "--verify-no-changes"])?;
    run_cargo(opts, &["fmt", "--all", "--", "--check"])?;
    Ok(())
}

pub fn format(opts: &BuildingOpts) -> eyre::Result<()> {
    run_cargo(opts, &["xtask", "update-version"])?;
    run_dotnet(opts, &["format"])?;
    run_cargo(opts, &["fmt", "--all"])?;
    Ok(())
}

pub fn run_tests(opts: &BuildingOpts) -> eyre::Result<()> {
    run_cargo(opts, &["test", "--workspace"])?;
    run_dotnet(opts, &["test"])?;
    Ok(())
}

pub fn run_lint(opts: &BuildingOpts) -> eyre::Result<()> {
    run_cargo(
        opts,
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_dotnet(opts, &["build", "-warnaserror"])?;
    Ok(())
}
