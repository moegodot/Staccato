use clap::Args;

use crate::BuildingOpts;

#[derive(Args, Debug, Clone)]
pub struct PreCommit {}
impl PreCommit {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct PrePush {}
impl PrePush {
    pub fn invoke(self, opts: &BuildingOpts) -> eyre::Result<()> {
        Ok(())
    }
}
