use serde_repr::{Serialize_repr, Deserialize_repr};
use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}, ToSql};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum Size {
    XS = 0, S = 1, M = 2, L = 3, XL = 4 
}

impl TryFrom<i32> for Size {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Size::XS),
            1 => Ok(Size::S),
            2 => Ok(Size::M),
            3 => Ok(Size::L),
            4 => Ok(Size::XL),
            _ => Err(()),
        }
    }
}

impl ToSql for Size {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl FromSql for Size {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let int_val = value.as_i64()? as i32;
        Size::try_from(int_val).map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl FromStr for Size {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XS" => Ok(Size::XS),
            "S" => Ok(Size::S),
            "M" => Ok(Size::M),
            "L" => Ok(Size::L),
            "XL" => Ok(Size::XL),
            _ => Err(()),
        }
    }
}