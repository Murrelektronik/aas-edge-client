## Example Structure

``` bash
src/
|-- main.rs           # Entry point, setup of the web server
|-- handlers/         # Request handlers
|   |-- mod.rs
|   `-- user.rs       # Example user-related handlers
|-- models/           # Data models
|   `-- user.rs
|-- routes.rs         # Route definitions
|-- error.rs          # Error types and handling
|-- db.rs             # Database access functions
|-- config.rs         # Configuration management
`-- utils.rs          # Utility functions
```

## Start MongoDB through Docker compose

``` bash
sudo docker compose up -d mongodb
```

## Start backend

### Development

``` bash
cargo run
```

### Specify the architecture with docker compose

``` bash
services:
  web:
    image: manhlinh210/rust_web_mongo:1.0.1 # <- change tag here
    platform: linux/amd64 # <- change architecture here: linux/arm64
```

### Build multiple image at parallel

Dont forget to change tag

``` bash
docker buildx build --platform linux/amd64,linux/arm64/v8 --tag manhlinh210/rust_web_mongo:1.0.1 --push .
```

## Branch

main: Update this when you have stable version

dev: Development branch

build: when you push on this branch, it will run the workflow, build and push docker image to docker hub
