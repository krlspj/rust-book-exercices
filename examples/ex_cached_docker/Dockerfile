FROM rust:1.86 AS build

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
	echo "fn main() { println!(\"I'm cool\") }" > src/main.rs && \
	cargo build --release && \
	rm -f target/release/ex_cached_docker
    #rm -rf src target/x86_64-unknown-linux-gnu/release/deps/conversation_closer*

ARG VERSION
ARG DATE
ENV VERSION=${VERSION}
ENV DATE=${DATE}

COPY src ./src
COPY build.rs ./
#COPY build.rs ./

RUN cargo build --release
#RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
#ENTRYPOINT ["/app/target/x86_64-unknown-linux-gnu/release/ex_cached_docker"]
CMD ["./target/release/ex_cached_docker"]
