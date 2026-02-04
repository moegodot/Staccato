use crate::fallible::Fallible;

pub trait HasSize {
    type SizeType;
    fn get_size(&self) -> Self::SizeType;
}

pub trait Resizable: HasSize + Fallible {
    fn try_set_size(&mut self, size: Self::SizeType) -> Result<(), Self::Error>;
}
