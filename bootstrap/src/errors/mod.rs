pub mod hookerr;
pub mod moderr;
pub mod conerr;

pub type DynErr = Box<dyn std::error::Error>;