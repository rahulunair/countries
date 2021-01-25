use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(long, short, default_value = "india")]
    pub country: String,
}
