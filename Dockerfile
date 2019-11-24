# FROM liuchong/rustup:nightly-musl as build
# COPY . /root
# RUN cargo build --release

# FROM scratch
# COPY --from=build /root/target/x86_64-unknown-linux-musl/release/qrmethis /qrmethis
# CMD ["/qrmethis"]
# EXPOSE 8000

FROM rust:1.39-buster as build

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release
RUN ldd target/release/canhaveinternet | awk '{ print $3 }' > libs.txt
RUN tar zcvf libs.tgz --files-from=libs.txt --dereference

FROM debian:buster
COPY --from=build /usr/src/myapp/libs.tgz /libs.tgz
RUN cd / && tar zxvf libs.tgz
COPY --from=build /usr/src/myapp/target/release/canhaveinternet /canhaveinternet
CMD ["/canhaveinternet"]

# CMD ["/usr/src/myapp/target/release/canhaveinternet"]
EXPOSE 8000
