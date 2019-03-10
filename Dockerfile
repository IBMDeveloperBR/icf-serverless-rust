# Dockerfile for example OpenWhisk docker action

FROM ekidd/rust-musl-builder AS builder

ADD . ./

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

FROM openwhisk/dockerskeleton

ENV FLASK_PROXY_PORT 8080

RUN apk --no-cache add ca-certificates

### Copy source file(s)
COPY --from=builder \
	/home/rust/src/target/x86_64-unknown-linux-musl/release/rusty_microservice \
	/action/exec

CMD ["/bin/bash", "-c", "cd actionProxy && python -u actionproxy.py"]
