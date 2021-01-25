use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(long, short, required = true, about = "country name")]
    pub name: String,
}
