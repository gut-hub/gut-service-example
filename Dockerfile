# use rust for the build container
FROM rust:1.53-slim-buster as rustc

# create empty rust project
RUN USER=root cargo new --bin jinx_service_example
WORKDIR /jinx_service_example

# copy over manifest files
COPY ./Cargo.toml ./Cargo.toml

# build dependencies (the dependencies will be cached unless the manifest files are updated)
RUN cargo build --release

# remove empty project files
RUN rm src/*.rs

# copy source
COPY ./src ./src

# build release
RUN touch -a -m ./src/main.rs
RUN cargo build --release

# use debian as the release container
FROM debian:buster-slim
WORKDIR /opt/jinx_service_example

# copy the config and binary
COPY --from=rustc /jinx_service_example/target/release/jinx-service-example .

# start application
CMD ["./jinx-service-example"]
