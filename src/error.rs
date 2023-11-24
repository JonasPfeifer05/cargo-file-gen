use std::fmt::{write, Display, Formatter};

#[derive(Debug)]
pub enum FileGenError {
    IncompatibleAsciiLoremError,
    InvalidAmountForSize,
    InvalidTypeForSize,
}

impl Display for FileGenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileGenError::IncompatibleAsciiLoremError => {
                write!(f, "You cannot pass --ascii and --lorem at the same time")
            }
            FileGenError::InvalidAmountForSize => {
                write!(f, "Invalid amount passed for the size")
            }
            FileGenError::InvalidTypeForSize => {
                write!(
                    f,
                    "Invalid type passed for size! Valid options are 'b', 'kb', 'mb' and 'gb'"
                )
            }
        }
    }
}
