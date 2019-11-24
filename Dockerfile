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
RUN ldd target/release/canhaveinternet | grep "=>" | awk '{ print $3 }' > libs.txt
RUN ldd target/release/canhaveinternet | grep -v "=>" | grep -v "linux-vdso" | awk '{ print $1 }' >> libs.txt
RUN tar zcvf libs.tgz --files-from=libs.txt --dereference
RUN mkdir dynamic_libs
RUN cd dynamic_libs && tar zxvf ../libs.tgz

FROM alpine:3
## copy across libraries used
COPY --from=build /usr/src/app/dynamic_libs/ /
## copy across binary
COPY --from=build /usr/src/app/target/release/canhaveinternet /canhaveinternet
ENTRYPOINT ["/canhaveinternet"]

EXPOSE 8000
