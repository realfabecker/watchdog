# base project image
FROM rust:1.75-slim-buster as base
RUN apt-get update && apt-get install -y make libssl-dev pkg-config
WORKDIR /app

# release build
FROM base as release
COPY . .
RUN make build-release

# binary output
FROM scratch as bundle
COPY --from=release /app/target ./target