use std::fmt::Display;


#[derive(Clone, Copy, Debug)]
pub enum NumericType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C
}

impl Display for NumericType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
        }
    }
}



impl TryFrom<u8> for NumericType {
    type Error = WasmErr;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            _ =>    Err(WasmErr::InvalidNumericTypeID(value))
        }
    }
}

impl Into<u8> for NumericType {
    fn into(self) -> u8 {
        match self {
            Self::I32 => 0x7F,
            Self::I64 => 0x7E,
            Self::F32 => 0x7D,
            Self::F64 => 0x7C,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WasmErr {
    InvalidNumericTypeID(u8)
}


#[cfg(test)]
mod tests {
    use crate::utils::types::NumericType;

    #[test]
    fn test_valid_numeric_type_from() {
        assert!(NumericType::try_from(0x7F).is_ok())
    }

    #[test]
    fn test_invalid_numeric_type_from() {
        assert!(NumericType::try_from(0x0).is_err())
    }
}