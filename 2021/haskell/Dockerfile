FROM docker.io/library/haskell:8.10.7@sha256:5c365dc081a22550cb4ca62f99a574f8583e8405689aa6a9d89fb21fa13724e9

# The stack shipped in old GHC images can no longer verify Hackage's signing
# keys, so install a current static stack release (checksums pinned).
ARG TARGETARCH
ARG STACK_VERSION=3.11.1
# The optional "extra-ca" secret lets `aoc` build behind a TLS-intercepting
# proxy (AOC_EXTRA_CA); without it the image's own CA bundle is used.
RUN --mount=type=secret,id=extra-ca,target=/etc/ssl/certs/ca-certificates.crt,required=false \
    case "$TARGETARCH" in \
      amd64) arch=x86_64;  sum=1fda71e657cd8d355625cc66b61b352699279dfee2664c014a392163bd19a952 ;; \
      arm64) arch=aarch64; sum=1617ae9976a5cd38ad4daec583b026b589eb45d5482afb045cd4ca8c8d0de6d0 ;; \
      *) echo "unsupported architecture $TARGETARCH" >&2; exit 1 ;; \
    esac \
    && curl -fsSL --cacert /etc/ssl/certs/ca-certificates.crt -o /tmp/stack.tgz "https://github.com/commercialhaskell/stack/releases/download/v$STACK_VERSION/stack-$STACK_VERSION-linux-$arch.tar.gz" \
    && echo "$sum  /tmp/stack.tgz" | sha256sum -c - \
    && tar -xzf /tmp/stack.tgz -C /usr/local/bin --strip-components=1 "stack-$STACK_VERSION-linux-$arch/stack" \
    && rm /tmp/stack.tgz

ENV STACK_ROOT=/opt/stack
RUN mkdir -p "$STACK_ROOT" && printf '%s\n' \
      'system-ghc: true' 'install-ghc: false' \
      'allow-different-user: true' 'recommend-stack-upgrade: false' \
      > "$STACK_ROOT/config.yaml"

WORKDIR /opt/deps
COPY stack.yaml stack.yaml.lock package.yaml ./
RUN --mount=type=secret,id=extra-ca,target=/etc/ssl/certs/ca-certificates.crt,required=false \
    mkdir -p src app test && stack build --only-dependencies \
    && rm -rf /opt/deps && chmod -R a+rwX "$STACK_ROOT"
WORKDIR /src
