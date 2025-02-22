FROM alpine:latest AS builder

COPY artifacts/binaries-x86_64-unknown-linux-musl/web-api   /web-api-amd64

COPY artifacts/binaries-aarch64-unknown-linux-musl/web-api /web-api-arm64

ARG TARGETARCH

RUN ARCH_FILE="/web-api-${TARGETARCH}"; \
    if [ -f "$ARCH_FILE" ]; then \
        cp "$ARCH_FILE" /usr/local/bin/web-api; \
    else \
        echo "Binary file for $TARGETARCH not found"; \
        exit 1; \
    fi

RUN chmod +x /usr/local/bin/web-api


FROM scratch

COPY --from=builder /usr/local/bin/web-api /web-api

CMD ["/web-api"]