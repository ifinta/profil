pub mod profile;
pub mod filter;
pub mod display;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Tab {
    #[default]
    Profile,
    Filter,
    Display,
}
