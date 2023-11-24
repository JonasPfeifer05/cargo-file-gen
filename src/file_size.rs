use crate::error::FileGenError;

pub struct FileSize {
    pub amount: u64,
    pub size: SizeType,
}

impl FileSize {
    pub fn byte_amount(&self) -> u64 {
        self.amount * self.size.to_byte_factor()
    }
}

impl TryInto<FileSize> for String {
    type Error = FileGenError;

    fn try_into(self) -> Result<FileSize, Self::Error> {
        let mut amount = String::new();
        let mut size_type = String::new();

        let mut amount_ended = false;
        for x in self.chars() {
            match x {
                '0'..='9' => {}
                _ => {
                    amount_ended = true;
                }
            }

            if amount_ended {
                size_type.push(x);
            } else {
                amount.push(x);
            }
        }

        let amount = match amount.parse::<u64>() {
            Ok(ok) => ok,
            Err(_err) => return Err(FileGenError::InvalidAmountForSizeError),
        };
        Ok(FileSize {
            amount,
            size: size_type.try_into()?,
        })
    }
}

pub enum SizeType {
    Byte,
    KiloByte,
    MegaByte,
    GigaByte,
}

impl SizeType {
    pub fn to_byte_factor(&self) -> u64 {
        match self {
            SizeType::Byte => 1,
            SizeType::KiloByte => 1024,
            SizeType::MegaByte => 1024 * 1024,
            SizeType::GigaByte => 1024 * 1024 * 1024,
        }
    }
}

impl TryInto<SizeType> for String {
    type Error = FileGenError;

    fn try_into(self) -> Result<SizeType, Self::Error> {
        let name = self.to_lowercase();
        let size_type = match name.as_str() {
            "b" => SizeType::Byte,
            "kb" => SizeType::KiloByte,
            "mb" => SizeType::MegaByte,
            "gb" => SizeType::GigaByte,
            _ => return Err(FileGenError::InvalidTypeForSizeError),
        };

        Ok(size_type)
    }
}
