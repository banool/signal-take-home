use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode, Uri};
use log::{debug, error, info};
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use structopt::{self, StructOpt};

type HttpClient = Client<hyper::client::HttpConnector>;

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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::from_args();

    debug!("Running with args: {:#?}", args);

    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let socket_address = SocketAddr::from((args.address, args.port));
    let server = Server::bind(&socket_address)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(service);

    info!("Listening on {}:{}", args.address, args.port);

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}

async fn proxy(_client: HttpClient, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Just return 404 for now.
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}
