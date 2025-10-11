use std::str::FromStr;

use crate::availability::Availability;

use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}, ToSql};
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use rand::{rng, seq::IndexedRandom};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, RusqliteStruct)]
pub struct Player {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub category: Category,
    pub date_of_creation: String,
    pub availability: Availability,
    pub size: Size,
    pub id_group: Option<i32>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub fn random() -> Self {
        const ALL: [Category; 8] = [
            Category::E,
            Category::D,
            Category::C,
            Category::B1,
            Category::B2,
            Category::A1,
            Category::A2,
            Category::Sconosciuta
        ];

        *ALL.choose(&mut rng()).unwrap()
    }

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