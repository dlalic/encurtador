# encurtador

Simple URL shortener server that tracks the number of visitors.

### Set up database

```
docker-compose up
```

### Set up environment

```
cp .env.template .env
```

### Run

```
cargo run
```

### Update OpenAPI spec

```
curl -o openapi.json http://localhost:3000/api/v1/openapi
```

### Update OpenAPI client for tests

```
openapi-generator-cli batch openapi-cli/rust.yaml
```
