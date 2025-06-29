use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
pub(crate) enum ColorMode {
    #[default]
    Filled,
    Wireframe,
    Both,
}
