use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Action {
    Sha(ShaCommand),
}

/// SHA-related utilities
#[derive(Parser, Debug)]
pub(crate) struct ShaCommand {
    #[clap(subcommand)]
    pub(crate) sub: ShaSubcommand,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum ShaSubcommand {
    Encode(ShaEncodeCommand),
    Decode(ShaDecodeCommand),
}

/// Encode a text with SHA-1
#[derive(Parser, Debug)]
pub(crate) struct ShaEncodeCommand {
    pub(crate) text: String,
}

/// Try to decode a SHA-1 encoded text with a wordlist
#[derive(Parser, Debug)]
pub(crate) struct ShaDecodeCommand {
    pub(crate) encoded_text: String,
    pub(crate) wordlist_path: String,
}
