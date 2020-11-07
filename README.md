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

If the `cargo install` command fails, use the following one:
```sh
cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres
```

If you have `make` isntalled, use `make run` to run dev server. Hot reload included.
To run those, you'll need to install following modules:

```sh
cargo install systemfd
cargo install cargo-watch
```

Not sure how to compile just yet, it likely wont even run either way lol.
No tests just yet. Not sure if ever will be.
