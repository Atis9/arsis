FROM rust:1.81-slim-bookworm AS runtime
RUN --mount=type=cache,id=api:/var/cache/apt,target=/var/cache/apt \
  --mount=type=cache,id=api:/var/lib/apt/lists,target=/var/lib/apt/lists \
  apt-get update && apt-get install --no-install-recommends -y \
  libopus-dev \
  libssl-dev \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

FROM runtime AS development
WORKDIR /usr/src/arsis

FROM runtime AS builder
WORKDIR /usr/src/arsis
COPY . .
RUN --mount=type=cache,target=/usr/src/arsis/target \
  cargo build --release --workspace \
  && cp target/release/arsis /arsis

FROM scratch AS production
LABEL io.github.atis9.arsis.app=arsis
LABEL org.opencontainers.image.source=https://github.com/Atis9/arsis
COPY --from=runtime /etc/ssl/certs/ /etc/ssl/certs/
COPY --from=runtime /lib/x86_64-linux-gnu/libc.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libcrypto.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libgcc_s.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libm.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libopus.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib/x86_64-linux-gnu/libssl.so* /lib/x86_64-linux-gnu/
COPY --from=runtime /lib64/ld-linux-x86-64.so* /lib64/
COPY --from=builder /arsis /
CMD ["/arsis"]
