# Toolchain for 2023/haskell: GHC 9.6.3 (as originally used) with the snapshot's
# Hackage dependencies pre-built, so `aoc` builds and runs need no network.
FROM docker.io/library/haskell:9.6.3@sha256:b9b9a9571466e025db499a077e3e8ce26934701713a5c5b7128507df5b529df1

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
# Each day is its own stack project; add new days' dependency files here.
COPY day7/stack.yaml day7/stack.yaml.lock day7/day7.cabal ./
RUN --mount=type=secret,id=extra-ca,target=/etc/ssl/certs/ca-certificates.crt,required=false \
    mkdir -p src app test && stack build --only-dependencies \
    && rm -rf /opt/deps && chmod -R a+rwX "$STACK_ROOT"
WORKDIR /src
