### ABOUT

Not much, hbu?

### INSTALLATION

First of all:

```sh
cp .env.example .env
```

After that, configure `.env` with your variables.

```sh
cargo install sqlx-cli --no-default-features --features postgres
sqlx database create
sqlx migrate run
cargo run
```

Not sure how to compile just yet, it likely wont even run either way lol.
No tests just yet. Not sure if ever will be.
