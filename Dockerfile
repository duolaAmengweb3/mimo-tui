# Multi-arch image for mimo-tui · pulls a pre-built linux binary for the
# target architecture instead of compiling inside the container.
#
# The CI workflow stages binaries as `mimo-x86_64` and `mimo-aarch64` in the
# build context before invoking `docker buildx build`.

FROM debian:bookworm-slim AS runtime

ARG TARGETARCH

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the right binary for this image's architecture.
COPY mimo-x86_64 /tmp/mimo-x86_64
COPY mimo-aarch64 /tmp/mimo-aarch64
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        mv /tmp/mimo-x86_64 /usr/local/bin/mimo; \
    else \
        mv /tmp/mimo-aarch64 /usr/local/bin/mimo; \
    fi \
    && rm -f /tmp/mimo-* \
    && chmod +x /usr/local/bin/mimo

WORKDIR /workspace

ENTRYPOINT ["/usr/local/bin/mimo"]
CMD ["--help"]
