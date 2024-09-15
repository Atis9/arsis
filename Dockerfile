# syntax = docker/dockerfile:1
FROM rust:1.81.0-bookworm AS base
WORKDIR /usr/src
COPY . ./

FROM base AS development
RUN --mount=type=cache,id=api:/usr/local/cargo/registry,target=/usr/local/cargo/registry \
  --mount=type=cache,id=api:/usr/src/target,target=/usr/src/target \
  cargo build && \
  cp -r target/debug/arsis /usr/local/bin/arsis

FROM base AS release
RUN --mount=type=cache,id=api:/usr/local/cargo/registry,target=/usr/local/cargo/registry \
  --mount=type=cache,id=api:/usr/src/target,target=/usr/src/target \
  cargo build --release && \
  cp -r target/release/arsis /usr/local/bin/arsis

FROM scratch AS production
LABEL io.github.atis9.arsis.app=arsis
LABEL org.opencontainers.image.source=https://github.com/Atis9/arsis
ENV PATH=/
COPY --link --from=release /lib/x86_64-linux-gnu/ld-linux-x86-64.* /lib/x86_64-linux-gnu/
COPY --link --from=release /lib/x86_64-linux-gnu/libc.so* /lib/x86_64-linux-gnu/
COPY --link --from=release /lib/x86_64-linux-gnu/libcrypto.so* /lib/x86_64-linux-gnu/
COPY --link --from=release /lib/x86_64-linux-gnu/libgcc_s.so* /lib/x86_64-linux-gnu/
COPY --link --from=release /lib/x86_64-linux-gnu/libm.so* /lib/x86_64-linux-gnu/
COPY --link --from=release /lib/x86_64-linux-gnu/libssl.so* /lib/x86_64-linux-gnu/
COPY --link --from=release /lib64 /lib64
COPY --link --from=release /usr/local/bin/arsis /arsis
COPY --link --from=release /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --link --from=release /usr/share/zoneinfo /usr/share/zoneinfo
RUN --mount=from=release,src=/bin,dst=/bin \
  --mount=from=release,src=/lib,dst=/lib \
  --mount=from=release,src=/usr,dst=/usr \
  /bin/mkdir --mode=1777 /tmp
ENTRYPOINT ["arsis"]
