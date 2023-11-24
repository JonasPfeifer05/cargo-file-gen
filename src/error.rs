use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FileGenError {
    IncompatibleArgumentsException,
}

impl Display for FileGenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileGenError::IncompatibleArgumentsException => {
                write!(f, "You cannot pass --ascii and --lorem at the same time")
            }
        }
    }
}
