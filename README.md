# API Users Auth

## Installation

Install postgres

```bash
# Macos
brew install postgres

# Linux
sudo apt-get install postgresql postgresql-contrib
```

Run database in docker

```bash
docker-compose up -d
```

Install diesel_cli

```bash
cargo install diesel_cli --no-default-features --features "postgres"
```

Copy env file and add database url

```bash
cp .env-sample .env
```

## TODO

- [ ] Login and register
- [ ] Add jwt auth
- [ ] Add refresh token
- [ ] Add tests
