#[derive(clap::Parser, Debug)]
pub struct Config {
    #[arg(name = "output", short, long, value_name = "FILE")]
    pub output_file: std::path::PathBuf,
    #[arg(short, long, value_name = "ADDR")]
    pub listen_addr: std::net::SocketAddr,
}
