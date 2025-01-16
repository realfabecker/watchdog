# base project image
FROM rust:1.75-slim-buster as base
RUN apt-get update && apt-get install -y make pkg-config git
WORKDIR /app

# release build
FROM base as release
COPY . .
RUN make build-release

# binary output
FROM scratch as bundle
COPY --from=release /app/target ./target