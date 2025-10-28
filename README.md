# Rust Short URL Service

⚡ **High-performance, cloud-ready URL shortening service built with Rust featuring distributed coordination, real-time analytics, and multiple storage backends including Firestore, MongoDB, and Redis.**

URL shortening service written in Rust. The service provides a basic API allowing clients to pass a long URL and receive a shortened version for easy access and sharing.

```
Usage: rust-short-url.exe [OPTIONS]

Options:
  -m, --mode <MODE>
          Mode this service will be running in [possible values: in-memory, persistent, coordinator]
  -a, --application-url <APPLICATION_URL>
          Address this service will be running on
      --hostnames <HOSTNAMES>
          List of host names separated by space to coordinate requests between
  -h, --help
          Print help
```
# Documentation

## Settings
Application settings can be configured in `development.toml` or `production.toml` files. Possible configurations include mode of the application, API server configuration, database connection, rate limiting and coordinator options.

### API Key Authentication
To secure admin endpoints, configure an API key in your settings file:
```toml
[apiserver]
application_url = "localhost:80"
hostname = "localhost"
allow_origin = "http://localhost:4200"
api_key = "your-secret-api-key-here"
```

If no `api_key` is configured, admin endpoints will be accessible without authentication (not recommended for production).

![Alt text](docs/settings.png?raw=true "Application settings")

## HashService
The `HashService` trait represents a service responsible for managing urls operations, such as initialization, retrieving links, inserting values, and finding information associated with a key.
![Alt text](docs/hashservice.png?raw=true "Hash Service")

## HTTP Server
HTTP server exposes several endpoints for the clients, such as:
* GET /urls - get all urls  
* GET /shorten?long_url{url} - shorten given long url
* GET /{short_url} - redirect to url behind this shortened version
* GET /{short_url}/summary - get summary information about provided short url

### Admin Endpoints (API Key Required)
Admin endpoints require authentication via API key in the `X-API-Key` header:
* GET /admin/urls - get all urls (requires API key authentication)
* DELETE /admin/{short_url} - delete a specific short url (requires API key authentication)

![Alt text](docs/httpserver.png?raw=true "HTTP Server")

## Statistics Collection
The service automatically collects analytics data whenever users navigate through short URLs. When a user visits a shortened URL (e.g., `/{short_url}`), the system captures and stores the following information:

* **Language**: Extracted from the `Accept-Language` HTTP header to determine the user's preferred language
* **Operating System**: Parsed from the `User-Agent` HTTP header using regex patterns to identify the user's operating system
* **IP Address**: Retrieved from `X-Forwarded-For` header (if behind a proxy) or `Remote-Addr` header
* **Geolocation**: Resolved from the IP address using an external geolocation API to determine the user's approximate location (city and country)
* **Timestamp**: Records when the short URL was accessed

This analytics data is stored with each URL entry and can be retrieved via the `/{short_url}/summary` endpoint, providing valuable insights into how and where the shortened URLs are being used. The statistics collection is transparent to users and doesn't affect the redirect performance.

## Data replication
The application can be launched in coordinator mode, coexisting with multiple regular instances within the same deployment. To enable coordination, a list of hostnames for these instances must be provided. In this mode, the application constructs a hash ring that encompasses all instances. Upon receiving a request, it forwards the request to all other machines, awaits acknowledgments from each, and then returns a response.

From the client's perspective, it appears as a single application. However, the underlying data is replicated across all instances. This replication enables fine-grained control over the consistency and availability of the data. When considering `N` as the number of replicas, `W` as the write quorum (considered successful if acknowledged by W replicas), and `R` as the read quorum (considered successful if acknowledged by R replicas), the following scenarios arise:

- If `R = 1` and `W = N`, the system is optimized for fast reads.
- If `W = 1` and `R = N`, the system is optimized for fast writes.
- If `W + R > N`, strong consistency is guaranteed (typically with `N = 3`, `W = R = 2`).
- If `W + R <= N`, strong consistency is not guaranteed.

![Alt text](docs/coordinator.png?raw=true "Data replication and coordinator")

# Run in Development

To run the service in development mode, use the following command:

```cmd
cargo run
```

By default, the `development.toml` configuration is used, and `RUN_MODE='development'` is set.

# Run in Production

Follow these steps to run the service in production:

* Configure MongoDB connection in `production.toml`.
* Set the environment variable `RUN_MODE` to 'production':
```
$env:RUN_MODE='production'
```
* Execute the following command:
```
cargo run
```

# Run in a Docker Container

To run the service using Docker Compose, use the following command:

```
docker-compose up
```

This command launches the service along with its dependencies defined in the docker-compose.yml file.

# Run in coordinator mode

To run multiple instances, use the following command:
```
00_start.ps1
```
Example output:
```
Instance 1 started on port 63975
Instance 2 started on port 63976
Instance 3 started on port 63977
Coordinator started on port 80
```

# Firestore Integration

This application supports integration with Google Firestore when the configuration parameter `Mode=Firestore` is set. To enable Firestore access, you need to set up application credentials with Google Cloud Platform (GCP). Follow these steps to configure and run the application:

---

## Steps to Configure Firestore Access

1. **Create a Service Account in GCP**
   - Navigate to the [Google Cloud Console](https://console.cloud.google.com).
   - Go to **IAM & Admin > Service Accounts**.
   - Click **Create Service Account**, provide a name and description, and proceed.
   - Grant the service account the necessary permissions for Firestore (e.g., **Cloud Datastore User** role).
   - Complete the creation process.

2. **Generate and Download a JSON Key**
   - In the Service Accounts list, locate your newly created service account.
   - Click on the service account, go to the **Keys** tab, and select **Add Key > Create New Key**.
   - Choose **JSON** format, then download and securely store the file (e.g., `urlshortener-445813-7cb90fbf70f8.json`).
  
3. **Save the Key File Locally**
   - Save the downloaded key file as `service-account.json` in a secure directory, e.g., `secrets`.  
   - Add `secrets/service-account.json` to your `.gitignore` to prevent it from being included in version control.  

4. **Map the JSON File in Docker Compose**
   - Modify your `compose.yml` to include the following configuration:  

        ```yaml
        volumes:
        # Map the JSON file from your local system to the container
        - ./secrets/service-account.json:/secrets/service-account.json:ro
        environment:
        - RUN_MODE=production
        - GOOGLE_APPLICATION_CREDENTIALS=/secrets/service-account.json
        ```

5. **Set the Application Credentials**
   You can set the `GOOGLE_APPLICATION_CREDENTIALS` in two ways:
   
   **Option A: Via Configuration File (Recommended)**
   - Add the path to your configuration file (e.g., `development.toml` or `production.toml`):
     ```toml
     [apiserver]
     GOOGLE_APPLICATION_CREDENTIALS = "D:\\Downloads\\urlshortener-445813-7cb90fbf70f8.json"
     ```
     The application will automatically set the environment variable from this configuration.
   
   **Option B: Via Environment Variable**
   - Set the environment variable manually in your terminal:
     ```powershell
     $env:GOOGLE_APPLICATION_CREDENTIALS="D:\Downloads\urlshortener-445813-7cb90fbf70f8.json"
     ```
     For other shells (e.g., bash):
     ```bash
     export GOOGLE_APPLICATION_CREDENTIALS="/path/to/urlshortener-445813-7cb90fbf70f8.json"
     ```
   
   **Note**: If both the environment variable and configuration file specify credentials, the environment variable takes precedence.

6. **Test locally**
   - Run the application using Cargo:
     ```bash
     cargo run
     ```

7. **Deploy the application**
        ```bash
        docker-compose -f compose.yaml up -d --build
        ```
---

# Usage

Execute a GET request to shorten a URL. Example:
```
curl -X GET "http://localhost/shorten?long_url=https://doc.rust-lang.org/"
```

The response will contain a short URL, e.g., `localhost/1C96D51A`.

You can now use `localhost/1C96D51A` as the shortened URL. Each time someone accesses this shortened URL, the service automatically collects anonymous analytics data (language, OS, IP address, and geolocation) for usage statistics.

## Admin Operations

### List all URLs (requires API key):
```bash
curl -X GET "http://localhost/admin/urls" \
  -H "X-API-Key: your-secret-api-key-here"
```

### Delete a short URL (requires API key):
```bash
curl -X DELETE "http://localhost/admin/1C96D51A" \
  -H "X-API-Key: your-secret-api-key-here"
```

# Cloud Deployment Architecture

This project demonstrates a cloud-based URL shortening service with an Angular frontend and Rust backend service deployed on Google Cloud Run. The architecture uses Cloudflare for frontend hosting, domains, routing and redirects. Below is an explanation of the deployment workflow and user interaction:

## Workflow

1. **User Interaction with Angular App**:
   - When a user visits [`https://surl.ivanenkomak.com/`](https://surl.ivanenkomak.com/), they are served the Angular frontend hosted behind Cloudflare.
   - The Angular app allows users to input a **Long URL** and click "Shorten" to generate a shortened URL.

2. **Shortened URL Generation**:
   - The Angular app generates and displays a **Short URL**, such as [`https://surl.ivanenkomak.com/9032D81A`](https://surl.ivanenkomak.com/9032D81A).
   - This Short URL includes a unique identifier (`9032D81A`) that maps to the original Long URL.

3. **Redirect via Cloudflare**:
   - When a user visits a Short URL [`https://surl.ivanenkomak.com/`](https://surl.ivanenkomak.com/), Cloudflare applies routing rules to differentiate between requests:
       - Requests to `/` are served the Angular app.
       - Requests to `/*` (paths with at least one character) are forwarded to the backend service.

4. **Backend Service on Google Cloud Run**:
   - The backend service, deployed on Google Cloud Run, receives the request with the unique identifier (`9032D81A`).
   - It resolves the identifier to the original Long URL and issues a redirect to the user.

The following diagram illustrates the architecture and workflow:

![Alt text](docs/deployment.png?raw=true "Cloud Deployment Workflow")

## Cloudflare Routing Rules

The following Cloudflare routing rules are used to handle traffic:

- **Root Path (`/`)**:
  - Requests to the root path serve the Angular app (`index.html`).
- **Short URLs (`/*`)**:
  - Requests to any path with a unique identifier are forwarded to the backend service hosted on Google Cloud Run.

## Example

1. **Long URL**:  
   `https://github.com/ivanenkomaksym/rust-short-url/blob/master/README.md`

2. **Short URL**:  
   `https://surl.ivanenkomak.com/9032D81A`

3. **Redirection**:
   - When the user visits the Short URL (`https://surl.ivanenkomak.com/9032D81A`), Cloudflare forwards the request to the backend.
   - The backend resolves the identifier (`9032D81A`) to the Long URL and redirects the user to `https://github.com/ivanenkomaksym/rust-short-url/blob/master/README.md`.

# References
Alex Xu, System Design Interview – An insider's guide, 2020