use anyhow::{bail, Error, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::upgrade::Upgraded;
use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
use log::{debug, info, warn};
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use structopt::{self, StructOpt};
use tokio::net::TcpStream;

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
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::from_args();
    debug!("Running with args: {:#?}", args);

    let mut allowed_hosts: Vec<String> = Vec::new();
    for ap in args.allowed_providers.iter() {
        match ap.host() {
            Some(host) => allowed_hosts.push(host.to_string()),
            None => bail!("Given provider does not have a host: {}", ap),
        }
    }

    let service = make_service_fn(move |_| {
        let ah = allowed_hosts.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(req, ah.clone()))) }
    });

    let socket_address = SocketAddr::from((args.address, args.port));
    let server = Server::bind(&socket_address)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(service);

    info!("Listening on {}:{}", args.address, args.port);

    server.await.map_err(|e| Error::new(e))
}

async fn proxy(
    req: Request<Body>,
    allowed_hosts: Vec<String>,
) -> Result<Response<Body>, hyper::Error> {
    // If the request is not a CONNECT request, return 501.
    if req.method() != Method::CONNECT {
        let mut response = Response::default();
        *response.status_mut() = StatusCode::NOT_IMPLEMENTED;
        warn!(
            "Rejected request for {} with method {} (should be CONNECT)",
            req.uri(),
            req.method()
        );
        return Ok(response);
    }

    let (authority, host) = match req.uri().authority() {
        Some(authority) => (authority.to_string(), authority.host().to_string()),
        None => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::BAD_REQUEST;
            warn!(
                "Requested URI {} does not contain an authority to connect to",
                req.uri(),
            );
            return Ok(response);
        }
    };

    if !allowed_hosts.contains(&host) {
        let mut response = Response::default();
        *response.status_mut() = StatusCode::BAD_REQUEST;
        warn!(
            "Rejected request for {} which is not in the list of allowed providers",
            host,
        );
        return Ok(response);
    }

    tokio::task::spawn(async move {
        match hyper::upgrade::on(req).await {
            Ok(upgraded) => {
                if let Err(e) = tunnel(upgraded, authority).await {
                    warn!("Server IO error: {}", e);
                };
            }
            Err(e) => warn!("Upgrade error: {}", e),
        }
    });

    Ok(Response::new(Body::empty()))
}

async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    // Connect to remote server now that all the validation is done.
    let mut server = TcpStream::connect(addr).await?;

    // Proxy data.
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    debug!(
        "Client wrote {} bytes and received {} bytes",
        from_client, from_server
    );

    Ok(())
}
