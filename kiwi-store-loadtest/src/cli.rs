use clap::Parser;

#[derive(Parser)]
#[command(
    name = "kiwi-store-loadtest",
    about = "A load/fuzz test client for the Kiwi Store",
    version = "1.0",
    author = "Burak Selim Şenyurt"
)]
pub struct Cli {
    #[arg(
        long,
        short,
        default_value = "load",
        help = "Type of test to run: load or fuzz"
    )]
    pub kind: String,
    #[arg(long, short, help = "Set the number of clients")]
    pub client_count: usize,
    #[arg(long, short, help = "Set the number of commands per client")]
    pub sample_count: usize,
}
