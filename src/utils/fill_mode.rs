use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
pub(crate) enum FillMode {
    #[default]
    Filled,
    Wireframe,
}
