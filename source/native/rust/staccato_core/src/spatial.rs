
pub trait HasSize {
    type SizeType;
    fn try_get_size(&mut self) -> Self::SizeType;
}

pub trait Resizable: HasSize {
    type Error: std::error::Error + Send + Sync + 'static;
    fn try_set_size(&mut self,size: Self::SizeType) -> Result<(),Self::Error>;
}
