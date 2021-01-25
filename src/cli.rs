use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    about = "Api to get information on countries",
    author = "unrahul <mail@rahul.onl>"
)]
pub struct Args {
    #[clap(long, short, required = true, about = "country name")]
    pub name: String,
}
