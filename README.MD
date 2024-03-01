# Start Project

### Create env

```shell
   cp .env.example .env
```

### Start

```shell
   docker-compose up -d
```

### Запусе cargo-watch c clippy

```shell
  cargo-watch -qc -x run -x clippy
```

## API
### Create link
```shell
curl --location 'http://127.0.0.1:8080/short-links' \
--header 'Content-Type: application/json' \
--data '{
    "link":"https://example.com"
}'
```
