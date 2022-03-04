# rust-api

Simple REST API built on rust using Rocket + Diesel (MySQL). Project consists of registration, login (using JWT) and
some other additional user modules

Build instructions:

1. Copy ../src/.env.example to ../src/.env

2. Run `docker-compose up --build`

3. Run `docker exec application bash -c "diesel setup && diesel migration run && cp src/schema.backup src/schema.rs"`

Import Rust API.postman_collection.json file in Postman
