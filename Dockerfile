FROM rust:1.39-buster as build

# prepare base image with dependencies
## create shell project
WORKDIR /usr/src/
RUN USER=root cargo new --bin app
WORKDIR /usr/src/app

## copy dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

## build and cache all dependencies
RUN cargo build --release

# build real app
## replace src
RUN rm src/*.rs
COPY ./src ./src

## build for release, using already compiled dependencies
RUN touch src/main.rs
RUN cargo build --release

# minimise down to what's needed to run
## find and store the libraries used by the app
RUN ldd target/release/canhaveinternet | awk '{ print $3 }' > libs.txt
RUN tar zcvf libs.tgz --files-from=libs.txt --dereference

FROM debian:buster-slim
# FROM rust:1.39-buster
## copy across libraries used
COPY --from=build /usr/src/app/libs.tgz /libs.tgz
RUN cd / && tar zxvf libs.tgz
## copy across ssl setup
COPY --from=build /etc/ssl/ /etc/ssl/
COPY --from=build /etc/ca-certificates.conf /etc/ca-certificates.conf  
COPY --from=build /etc/ca-certificates/ /etc/ca-certificates/
## copy across binary
COPY --from=build /usr/src/app/target/release/canhaveinternet /canhaveinternet
CMD ["/canhaveinternet"]

EXPOSE 8000
