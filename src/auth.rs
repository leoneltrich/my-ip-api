use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier
    },
    Argon2
};
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response
};
use std::env;
use std::time::Duration;

pub(crate) async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Load expected token from env
    let expected_token = env::var("ACCESS_TOKEN").map_err(|_| { 
        println!("Access token not found in environment");
        StatusCode::INTERNAL_SERVER_ERROR 
    })?;

    // Extract the Authorization header
    let headers = req.headers();
    let token = extract_bearer_token(headers).ok_or(StatusCode::UNAUTHORIZED)?;

    println!("Expected {}", expected_token);

    // Check if the token matches
    let parsed_hash = PasswordHash::new(&expected_token).map_err(|e| {
        println!("Failed to parse password hash: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !Argon2::default().verify_password(token.as_bytes(), &parsed_hash).is_ok() {
        println!("Access token mismatch");
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    println!("Access token valid");
    Ok(next.run(req).await)
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("authorization")?.to_str().ok()?;
    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        Some(token.to_string())
    } else {
        None
    }
}
