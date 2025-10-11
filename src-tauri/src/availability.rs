use bitflags::bitflags;
use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}, ToSql};
use serde::{Serialize, Deserialize};

// TODO: Aggiungere possibilmente con una proc macro la possibilità di aggiungere altre fasce orarie e fare velocemente 
//       il cast a string ottenendo i nomi dei field a runtime soprattutto per la creazione dei form e il get delle loro info.
bitflags! {
    #[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
    pub struct Availability: i32 {
        const None   = 0b000;
        const SatAft = 0b001;
        const SunMor = 0b010;
        const SunAft = 0b100;
    }
}

impl ToSql for Availability {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(self.bits() as i64)))
    }
}

impl FromSql for Availability {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let int_val = value.as_i64()? as i32;
        Ok(Availability::from_bits_truncate(int_val))
    }
}

impl Availability {
    pub fn all_slots() -> [Availability; 3] {
        [Availability::SatAft, Availability::SunMor, Availability::SunAft]
    }
    
    pub fn slots(&self) -> impl Iterator<Item = Availability> {
        Self::all_slots()
            .into_iter()
            .filter(|flag| self.contains(*flag))
    }
}

impl<'de> Deserialize<'de> for Availability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let bits = i32::deserialize(deserializer)?;
        Availability::from_bits(bits).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid bits for Availability: {}", bits))
        })
    }
}