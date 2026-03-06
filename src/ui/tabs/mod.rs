pub mod profile;
pub mod filter;
pub mod display;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Tab {
    #[default]
    Profile,
    Filter,
    Display,
}
