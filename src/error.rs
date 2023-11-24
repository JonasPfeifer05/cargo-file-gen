use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FileGenError {
    IncompatibleAsciiLoremError,
    InvalidSizeError,
}

impl Display for FileGenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileGenError::IncompatibleAsciiLoremError => {
                write!(f, "You cannot pass --ascii and --lorem at the same time")
            }
            FileGenError::InvalidSizeError => {
                write!(f, "The size you inputted was wrongly formatted! Valid options are 'b', 'kb', 'mb' and 'gb'")
            }
        }
    }
}
