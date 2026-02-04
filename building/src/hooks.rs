use crate::{BuildingOpts, actions};
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct PreCommit {}
impl PreCommit {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        actions::check_well_format(opts)?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct PrePush {}
impl PrePush {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        actions::run_lint(opts)?;
        actions::run_tests(opts)?;
        Ok(())
    }
}
