FROM rust:1.75-alpine3.19 as builder


# Feature apenas em nightly
RUN rustup update nightly

RUN rustup default nightly

# Definindo a pasta dentro do container onde
WORKDIR /usr/src/tech-challenge

# Install necessary packages for building Rust applications and OpenSSL development headers
RUN apk add musl-dev openssl-dev

COPY . .

RUN cargo build --release

# Criando um novo stage com imagem scratch e passando apenas os 
# arquivos necessarios
FROM scratch as production
COPY --from=builder /usr/src/tech-challenge/target/release /
ENTRYPOINT [ "/api" ]
EXPOSE 3000