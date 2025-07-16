use crate::auth::auth_middleware;
use axum::routing::get;
use axum::{Json, Router, middleware};
use serde::Serialize;
use std::net::IpAddr;

mod auth;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/ip", get(get_current_ip))
        .layer(middleware::from_fn(auth_middleware));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Debug)]
struct IpResponse {
    ipv4: Option<String>,
    ipv6: Option<String>,
}


async fn get_current_ip() -> Json<IpResponse> {
    println!("Getting current IP");

    // Create a client that we'll reuse for both requests
    let client = reqwest::Client::new();

    // Make requests to services that can return IPv4 and IPv6 addresses
    // We'll use different services to increase the chance of getting both types
    let ipv4_future = client.get("https://api.ipify.org").send();
    let ipv6_future = client.get("https://api64.ipify.org").send();

    // Wait for both requests to complete
    let (ipv4_result, ipv6_result) = tokio::join!(ipv4_future, ipv6_future);

    // Process IPv4 result
    let ipv4 = match ipv4_result {
        Ok(res) => {
            match res.text().await {
                Ok(text) => {
                    if let Ok(addr) = text.parse::<IpAddr>() {
                        if let IpAddr::V4(ipv4_addr) = addr {
                            Some(ipv4_addr.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                Err(_) => None,
            }
        },
        Err(_) => None,
    };

    // Process IPv6 result
    let ipv6 = match ipv6_result {
        Ok(res) => {
            match res.text().await {
                Ok(text) => {
                    if let Ok(addr) = text.parse::<IpAddr>() {
                        if let IpAddr::V6(ipv6_addr) = addr {
                            Some(ipv6_addr.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                Err(_) => None,
            }
        },
        Err(_) => None,
    };

    // Create response with both IP addresses
    let response = IpResponse { ipv4, ipv6 };
    println!("Response: {:?}", response);

    Json(response)
}
