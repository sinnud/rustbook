# The First Rust Project
Use postgresql data base to handle WD net drives.

## functions
- Refresh table in postgresql based on file structure on WD net drives.
- Sync two WD net drives based on database.
- Check out [doc(https://sinnud.github.io/rust_wdinfo/doc/settings.html)] for documentation.

## Interesting Rust skill
- Use rust library structure. See Cargo.toml
- Accept command line arguments
- Use log4rs with yaml setting
  - release production with `config/log4rs.yaml` same folder with executable file
  - in `config/log4rs.yaml`, set log file with absolute path such that it does not depend on where to run code.
- Use progres crate to connect to PostgreSQL data base
- Get file status
- auto create doc (try to see if we can publish doc in GIT)

## useful command need to be noted
- Build development version `cargo build`
- Build release version `cargo build --release`
- Build library only `cargo build --lib`
- Build executable file only `cargo build --bins`
- Build and run `cargo run`
- Build documentation (only my code) `cargo doc --no-deps`