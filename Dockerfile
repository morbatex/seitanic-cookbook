FROM rustlang/rust:nightly-buster
WORKDIR /usr/src/app
COPY . /usr/src/app
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /seitanic-cookbook
ENTRYPOINT ["./seitanic-cookbook"]
RUN apt-get update && apt-get install libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=0 /usr/src/app/target/release/seitanic-cookbook /usr/src/app/Rocket.toml ./
EXPOSE 80
