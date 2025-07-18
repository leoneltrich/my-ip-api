use std::net::IpAddr;
use axum::Json;
use reqwest::{Client, Error, Response};
use crate::models::ip_response::IpResponse;

pub async fn get_current_ip() -> Json<IpResponse> {
    println!("Getting current IP");

    // Create a client that we'll reuse for both requests
    let client = reqwest::Client::new();

    // Send API request to get IPv4 & IPv6 addresses
    let (ipv4_result, ipv6_result) = send_requests(client).await;

    // Process results
    let ipv4 = process_ipv4(ipv4_result).await;
    let ipv6 = process_ipv6(ipv6_result).await;

    // Create response with both IP addresses
    let response = IpResponse { ipv4, ipv6 };
    println!("Response: {:?}", response);

    Json(response)
}

async fn send_requests(client: Client) -> (Result<Response, Error>, Result<Response, Error>) {
    let ipv4_future = client.get("https://api.ipify.org").send();
    let ipv6_future = client.get("https://api64.ipify.org").send();

    // Wait for both requests to complete
    let (ipv4_result, ipv6_result) = tokio::join!(ipv4_future, ipv6_future);
    (ipv4_result, ipv6_result)
}

async fn process_ipv6(ipv6_result: Result<Response, Error>) -> Option<String> {
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
    ipv6
}

async fn process_ipv4(ipv4_result: Result<Response, Error>) -> Option<String> {
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
    ipv4
}