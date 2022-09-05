//! Default Compute@Edge template program.

use fastly::geo::geo_lookup;
use fastly::http::{header, Method, StatusCode};
use fastly::{Error, Request, Response};

use chrono::{Utc};
use serde_json::json;

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    // Pattern match on the path...
    match req.get_path() {
        // If request is to the `/` path...
        "/" => {
            let mut resp = Response::from_status(StatusCode::OK);
            let client_ip = req.get_client_ip_addr().unwrap();
            let geo = fastly::geo::geo_lookup(client_ip).unwrap();

            log_fastly::init_simple("my_endpoint", log::LevelFilter::Info);

            // Get some data to log
            let record = json!({
                "timestamp": Utc::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string(),
                "trace_id": std::env::var("FASTLY_TRACE_ID").unwrap_or_else(|_| String::new()),
                "client_ip": client_ip.to_string(),
                "geo_country": geo.country_name(),
                "geo_city": geo.city(),
                "host": req.get_header_str("Host"),
                "request_method": req.get_method_str(),
                "url": req.get_url_str(),
                "request-referer": req.get_header_str("Referer"),
                "request_user_agent": req.get_header_str("User-Agent"),
                "response_status": resp.get_status().as_u16(),
                "response_reason": resp.get_status().canonical_reason(),
                "fastly_server": std::env::var("FASTLY_POP").unwrap_or_else(|_| String::new()),
                "fastly_hostname": std::env::var("FASTLY_HOSTNAME").unwrap_or_else(|_| String::new()),
                "fastly_service_id": std::env::var("FASTLY_SERVICE_ID").unwrap_or_else(|_| String::new()),
                "fastly_service_version": std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new()),
            });

            log::info!("{}", record);

            // This message will show in the log-tail
            println!("{}", record);
 
            // Send the JSON in the response
            resp.set_body_json(&record).unwrap();     

            // Send the response
            return Ok(resp);
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
