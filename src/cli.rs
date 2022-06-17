use clap::Parser;

const ARG_GOD_HELP: &str =
    "Makes the snake invincible and shows some debug information";

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short, long, help = ARG_GOD_HELP)]
    pub(crate) debug: bool,
}
