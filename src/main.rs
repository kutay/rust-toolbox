use clap::Parser;

mod cli;
mod sha;

use cli::*;

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Sha(args) => match args.sub {
            ShaSubcommand::Encode(args) => {
                let encoded = sha::encode_sha1(&args.text);
                println!("{:?}", encoded);
            }
            ShaSubcommand::Decode(args) => {
                let encoded = sha::decode_sha1_string(&args.wordlist_path, &args.encoded_text);
                println!("{:?}", encoded);
            }
        },
    }
}
