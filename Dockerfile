# Local CPU-only amd64 build. See docs/licensing.md before redistributing binaries.
FROM rust:1.90.0-bookworm@sha256:3914072ca0c3b8aad871db9169a651ccfce30cf58303e5d6f2db16d1d8a7e58f AS build
ENV SHERPA_ONNX_LIB_DIR=/opt/sherpa/lib
WORKDIR /build
RUN curl --fail --location --retry 3 https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.13.8/sherpa-onnx-v1.13.8-linux-x64-static-lib.tar.bz2 -o /tmp/sherpa.tar.bz2 \
 && echo 'e1fdc5b67530e15741ef897fa5ffff297056f3bf0c6d829a27af9225a4c4b5a6  /tmp/sherpa.tar.bz2' | sha256sum --check \
 && mkdir -p /opt/sherpa \
 && tar -xjf /tmp/sherpa.tar.bz2 --strip-components=1 -C /opt/sherpa
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo test --locked && cargo build --release --locked --bins
FROM gcr.io/distroless/cc-debian12@sha256:777e96cf322c46bc32aca926c263624c4dc8d7cf37e2fa65ba2c7e697318ebbb
COPY --from=build /build/target/release/kroko-wyoming /usr/local/bin/kroko-wyoming
COPY --from=build /build/target/release/wyoming-probe /usr/local/bin/wyoming-probe
COPY --from=build /usr/share/zoneinfo/Europe/Berlin /usr/share/zoneinfo/Europe/Berlin
COPY LICENSE NOTICE /usr/share/licenses/kroko/
USER 65532:65532
ENV KROKO_MODEL=classic MODEL_DIR=/models HOST=0.0.0.0 PORT=10321 NUM_THREADS=1 LANGUAGE=de LOG_LEVEL=info TZ=Europe/Berlin
EXPOSE 10321
HEALTHCHECK --interval=15s --timeout=5s --start-period=30s --retries=3 CMD ["/usr/local/bin/wyoming-probe"]
ENTRYPOINT ["/usr/local/bin/kroko-wyoming"]
