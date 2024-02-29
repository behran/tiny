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
