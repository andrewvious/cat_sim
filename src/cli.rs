use clap::*;

#[derive(Debug, Parser, PartialEq)]
#[clap(
    name = "cat_sim",
    version = "1.1",
    about = "You have a cat, take care of them!"
)]
pub struct KittyArgs {
    #[clap(subcommand)]
    pub kitty_method: MethodType,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum MethodType {
    /// Creates a new kitty to take care of!
    CreateKitty { color: String },
    /// Feed your kitty!
    FeedKitty { feed: String },
    /// Pat your kitty!
    GivePat { pat: String },
    /// Give your kitty a wittle tweat!
    GiveTreat { treat: String },
}
