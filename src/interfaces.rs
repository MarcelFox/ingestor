
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UseUnit {
    Cpu,
    Memory,
    Storage,
}

#[derive(Serialize, Deserialize)]
pub struct PulseData {
    pub tenant: String,
    pub product_sku: String,
    pub used_amount: f64,
    pub use_unit: UseUnit,
    pub timestamp: Option<String>,
}

impl fmt::Display for UseUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            UseUnit::Cpu => "cpu",
            UseUnit::Memory => "memory",
            UseUnit::Storage => "storage",
        };
        write!(f, "{}", s)
    }
}
