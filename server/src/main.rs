use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;


#[derive(Debug)]

struct RateLimiter {
    visitors: Arc<Mutex<HashMap<SocketAddr, u32>>>,
    }


impl RateLimiter {
    fn new() -> Self { 
        RateLimiter {
        visitors: Arc::new(Mutex::new(HashMap::new())), 
    }}
    fn allow(&self, addr: SocketAddr) -> bool {
    let mut visitors = self.visitors.lock().unwrap(); 
    let counter = visitors.entry(addr).or_insert(0); 
    if *counter >= 5 {
        false
    } else {
        *counter += 1;
        true
    } 
}
}


async fn handle_request(req: Request<Body>, rate_limiter: Arc<RateLimiter>, addr: SocketAddr) -> Result<Response<Body>, hyper::Error> {
    match req.uri().path() {
        "/service1" => {
            let mut response = Response::new(Body::from("Hello from service1"));
            *response.status_mut() = hyper::StatusCode::CREATED;
            Ok(response)
        },
        "/service2" => {
            let mut response = Response::new(Body::from("Hello from service2"));
            *response.status_mut() = hyper::StatusCode::CREATED;
            Ok(response)
        },
        "/service3" => {
            println!("{}" , addr);
            println!("{}" , rate_limiter.allow(addr));

            if !rate_limiter.allow(addr) {
                return Ok(Response::new(Body::from("Rate limit exceeded for service3")))
            } 

            let client = hyper::Client::new();
            let forwarded_req = Request::builder().method(req.method()).uri("http://userservice:3000/api/v1/users/allusers").body(req.into_body())
            .unwrap();
            let mut resp: Response<Body> = client.request(forwarded_req).await?;
            *resp.status_mut() = hyper::StatusCode::CREATED;
            Ok(resp)
        },
        _ => Ok(Response::new(Body::from("Not Found"))), 
    }
}


#[tokio::main]
async fn main() {
    let rate_limiter = Arc::new(RateLimiter::new());
    
    let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
        let rate_limiter = rate_limiter.clone();
        let address = conn.remote_addr();
        let service = service_fn(move |req| handle_request(req, rate_limiter.clone(), address));
        async move { Ok::<_, hyper::Error>(service) }
    });
    let addr = ([0, 0, 0, 0], 10000).into();
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}