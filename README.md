# 🦀 Rust API (REST)

## An introduction to rust

## Background
I decided to learn rust by building a simple REST API. Currently, the plan is to learn how to build a simple REST interface using the language without introducing any complexities such as databases, caching, or authentication, etc. This project is a web server that exposes a few endpoints doing dummy CRUD operations. The next step would be to ~~introduce OpenAPI docs generation~~ (done), and then dockerise & ~~deploy it to the cloud~~ (deployed).

## How to run the project locally?
1. Make sure you have rust installed
2. Clone the repo locally
3. Run this command in terminal: `cargo run` - it starts the server
4. To test the APIs locally, either use the [postman collection](./postman/) or use the auto-generated OpenAPI docs by visiting `http://localhost:8080/swagger-ui/`

## How to try out the API using our postman collection?
1. Open (or download & install) Postman
2. Import the API collection from [postman/collection.json](./postman/collection.json)
3. Import [postman/environment.json](./postman/environment.json) as your new environment
