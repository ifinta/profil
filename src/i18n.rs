use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Language {
    English,
    #[default]
    Hungarian,
    German,
    French,
    Finnish,
    Spanish,
    Greek,
    Italian,
}
