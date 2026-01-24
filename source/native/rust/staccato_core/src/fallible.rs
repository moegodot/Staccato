use std::error::Error;

pub trait Fallible{
    type Error : Error + Sync + Send + 'static;
}
 