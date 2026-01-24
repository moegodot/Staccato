use std::fmt::Debug;
use crate::fallible::Fallible;

/// 定义逻辑生命周期
pub trait Tickable : Fallible + Debug{
    /// 准备阶段：处理输入、同步网络、重置状态
    fn pre_update(&mut self, elapse_ns:u64) -> Result<(),Self::Error>;

    /// 物理阶段：固定步长更新
    fn fixed_update(&mut self, elapse_ns:u64) -> Result<(),Self::Error>;

    /// 逻辑阶段：处理业务逻辑
    fn update(&mut self, elapse_ns:u64) -> Result<(),Self::Error>;

    /// 后处理阶段：相机跟随、动画同步、渲染前最后的调整
    fn post_update(&mut self, elapse_ns:u64) -> Result<(),Self::Error>;
}
