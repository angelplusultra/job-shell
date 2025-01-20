use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable Discord mode
    #[arg(long)]
    pub discord: bool,
}
