use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct AppArgs {
    #[clap(short, long)]
    username: String,
    #[clap(short, long)]
    host: String,
    #[clap(short, long)]
    password: String,
}
