FROM jimmycuadra/rust
EXPOSE 8080
COPY Cargo.toml /source
COPY src/main.rs /source/src/
CMD cargo run

