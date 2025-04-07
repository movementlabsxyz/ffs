FROM nixos/nix:2.28.0 AS builder

RUN nix-env -iA nixpkgs.rsync nixpkgs.glibc nixpkgs.gawk

# Copy the source code into the container
COPY . /tmp/build
WORKDIR /tmp/build

# Build the Rust application
RUN nix --extra-experimental-features "nix-command flakes" \
        develop .#docker-build --command bash -c "cargo build --release -p mcr-network-client"

# Handle dynamic linking
RUN rust_binary="./target/release/mcr-network-client"; dest_dir="/tmp/runtime"; \
    mkdir -p "$dest_dir"; ldd "$rust_binary" | awk '{print $3}' | \
    grep '^/' | xargs -I {} dirname {} | sort | uniq | xargs -I {} \
    bash -c 'mkdir -p "$0/$1" && rsync -a --copy-links "$1/" "$0/$1/"' "$dest_dir" {}

FROM alpine:3.21.3

# Copy the build artifact from the builder stage
COPY --from=builder /tmp/build/target/release/mcr-network-client /app/mcr-network-client
COPY --from=builder /tmp/runtime/nix/store /nix/store

# Set the binary as the entrypoint
ENTRYPOINT ["/app/mcr-network-client"]
LABEL org.opencontainers.image.source https://github.com/movementlabsxyz/ffs