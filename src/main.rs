use hyper::Uri;
use std::net::IpAddr;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Address to run the server on.
    #[structopt(long, alias = "ip")]
    address: IpAddr,

    /// Port to bind to.
    #[structopt(short, long)]
    port: u16,

    /// Providers that we will allow users to make requests to via this proxy.
    #[structopt(long, required = true)]
    allowed_providers: Vec<Uri>,
}


#[tokio::main]
async fn main() {
    let args = Args::from_args();

    println!("args: {:#?}", args);
}
