use serde_repr::{Serialize_repr, Deserialize_repr};
use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}, ToSql};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum Category {
    Sconosciuta = 0, E = 1, D = 2, C = 3, B1 = 4, B2 = 5, A1 = 6, A2 = 7
}

impl TryFrom<i32> for Category {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Category::Sconosciuta),
            1 => Ok(Category::E),
            2 => Ok(Category::D),
            3 => Ok(Category::C),
            4 => Ok(Category::B1),
            5 => Ok(Category::B2),
            6 => Ok(Category::A1),
            7 => Ok(Category::A2),
            _ => Err(()),
        }
    }
}

impl ToSql for Category {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl FromSql for Category {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let int_val = value.as_i64()? as i32;
        Category::try_from(int_val).map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl Category {
    pub fn all_categories() -> [Category; 8] {
        use Category::*;
        [Sconosciuta, E, D, C, B1, B2, A1, A2]
    }

    pub fn to_string(&self) -> String {
        match self {
            Category::E => "E".to_string(),
            Category::D => "D".to_string(),
            Category::C => "C".to_string(),
            Category::B1 => "B1".to_string(),
            Category::B2 => "B2".to_string(),
            Category::A1 => "A1".to_string(),
            Category::A2 => "A2".to_string(),
            Category::Sconosciuta => "Sconosciuta".to_string()
        }
    }
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" => Ok(Category::E),
            "D" => Ok(Category::D),
            "C" => Ok(Category::C),
            "B1" => Ok(Category::B1),
            "B2" => Ok(Category::B2),
            "A1" => Ok(Category::A1),
            "A2" => Ok(Category::A2),
            "Sconosciuta" => Ok(Category::Sconosciuta),
            _ => Err(()),
        }
    }
}