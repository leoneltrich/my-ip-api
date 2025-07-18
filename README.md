# IP Lookup Service

A simple Rust-based API service that returns the IPv4 and IPv6 addresses of the machine it is running on. The service is built with Axum and Tokio, and includes Docker support for easy deployment.

## Features

- Returns both IPv4 and IPv6 addresses when available
- Secure API with token-based authentication
- Lightweight Docker image (~8MB)
- Built with Rust for performance and reliability

## Prerequisites

- Rust (nightly edition) for local development
- Docker and Docker Compose for containerized deployment

## Environment Variables

The service requires the following environment variables:

| Variable       | Description                                                                           | Default                                       |
|----------------|---------------------------------------------------------------------------------------|-----------------------------------------------|
| `PORT`         | The port on which the service will listen                                             | `3000`                                        |
| `ACCESS_TOKEN` | Authentication token (Argon2 hashed, create your hash [here](https://argon2.online/)) | `default_token_change_me` (set in Dockerfile) |

## Setup

### Local Development

1. Clone the repository
2. Set the required environment variables:
   ```bash
   export PORT=3000
   export ACCESS_TOKEN="$argon2id$v=19$m=19456,t=2,p=1$Q2xLdkxDRURkRVg4Z1UxSA$nbp8PT3rTnHL79D++2RtXQ" # hash of the token changeme
   ```
3. Build and run the project:
   ```bash
   cargo build --release
   ./target/release/ip_lookup
   ```

### Docker Deployment (direct)

1. Clone the repository and change into directory
2. Build the image
   ```bash
   docker build -t my-ip-api .
   ```
3. Run the image with environment variables:
   ```bash
   docker run -e ACCESS_TOKEN="your_hashed_token" -e PORT=3000 my-ip-api
   ```
   
### Docker Deployment (docker compose)
1. Set environment variables in docker-compose.yml
2. Build and run with Docker Compose:
   ```bash
   docker-compose up -d
   ```

## API Endpoints

### GET /ip

Returns the IPv4 and IPv6 addresses of the server.

#### Authentication

This endpoint requires authentication using a Bearer token in the Authorization header.

#### Request

```http
GET /ip
Authorization: Bearer your_token
```

#### Response

```json
{
  "ipv4": "203.0.113.1",
  "ipv6": "2001:db8::1"
}
```

Note: Either `ipv4` or `ipv6` may be `null` if the address is not available.

## Example Usage

### Using curl

```bash
curl -H "Authorization: Bearer your_token" http://localhost:3000/ip
```

### Using JavaScript (fetch)

```javascript
const response = await fetch('http://localhost:3000/ip', {
  headers: {
    'Authorization': 'Bearer your_token'
  }
});
const data = await response.json();
console.log(data);
```

## Security Considerations

- Always change the default ACCESS_TOKEN when deploying to production
- The service uses Argon2 for secure token verification
- There's a 1-second delay in authentication to mitigate brute force attacks

## Building the Docker Image

The project includes a Dockerfile that creates a minimal image based on scratch:

```bash
docker build -t ip-lookup .
```

The resulting image is approximately 8MB in size.

## License

See the [LICENSE](LICENSE) file for details.
