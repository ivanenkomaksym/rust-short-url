# Rust Short URL Service

Simple URL shortening service written in Rust. The service provides a basic API allowing clients to pass a long URL and receive a shortened version for easy access and sharing.

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
![Alt text](docs/httpserver.png?raw=true "HTTP Server")

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
   - In your terminal, set the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to the path of your JSON key file:
     ```powershell
     $env:GOOGLE_APPLICATION_CREDENTIALS="D:\Downloads\urlshortener-445813-7cb90fbf70f8.json"
     ```
     For other shells (e.g., bash):
     ```bash
     export GOOGLE_APPLICATION_CREDENTIALS="/path/to/urlshortener-445813-7cb90fbf70f8.json"
     ```

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

Execute a POST request to shorten a URL. Example:
```
curl -X GET "http://localhost/shorten?long_url=https://doc.rust-lang.org/"
```

The response will contain a short URL, e.g., `localhost/1C96D51A`.

You can now use `localhost/1C96D51A` as the shortened URL.

# References
Alex Xu, System Design Interview – An insider's guide, 2020