{ pkgs, lib, ... }:

{
  packages = with pkgs; [
    # Rust toolchain
    rustup
    cargo-edit
    cargo-watch
    cargo-nextest
    cargo-tarpaulin
    cargo-audit
    cargo-deny
    cargo-flamegraph
    cargo-make

    # Build tools
    pkg-config
    openssl
    sqlite

    # Code quality
    nixpkgs-fmt
    treefmt
    typos

    # Utilities
    just
    mdbook
  ];

  env = {
    RUSTUP_HOME = "$HOME/.rustup";
    CARGO_HOME = "$HOME/.cargo";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.sqlite.dev}/lib/pkgconfig";
    LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.openssl pkgs.sqlite pkgs.stdenv.cc.cc.lib ];
  };

  scripts = {
    build.exec = "cargo build --workspace";
    test.exec = "cargo test --workspace";
    check.exec = "cargo check --workspace && cargo clippy --workspace -- -D warnings";
    lint.exec = "typos && nix fmt";
    coverage.exec = "cargo tarpaulin --workspace --out Html";
    run-dev.exec = "cargo run -p blackbox-dev";
  };

  enterShell = ''
    echo "Blackbox — Lunar Tear Rust Rewrite"
    echo "==================================="
    echo ""
    echo "Commands:"
    echo "  build    — cargo build --workspace"
    echo "  test     — cargo test --workspace"
    echo "  check    — cargo check + clippy"
    echo "  lint     — typos + nix fmt"
    echo "  coverage — cargo tarpaulin"
    echo "  run-dev  — cargo run -p blackbox-dev"
    echo ""

    # Ensure rustup default stable is installed
    if ! rustup show active-toolchain &>/dev/null; then
      echo "Installing Rust stable toolchain..."
      rustup toolchain install stable
    fi
  '';
}
