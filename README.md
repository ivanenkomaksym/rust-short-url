# Rust Short URL Service

Simple URL shortening service written in Rust. The service provides a basic API allowing clients to pass a long URL and receive a shortened version for easy access and sharing.

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

# Usage

Execute a POST request to shorten a URL. Example:
```
curl -X GET "http://localhost/shorten?long_url=https://doc.rust-lang.org/"
```

The response will contain a short URL, e.g., `localhost/1C96D51A`.

You can now use `localhost/1C96D51A` as the shortened URL.
