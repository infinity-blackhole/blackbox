{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-darwin" "x86_64-linux" "aarch64-darwin" "aarch64-linux" ];
      perSystem = { pkgs, ... }: {
        devShells.default = pkgs.mkShell {
          name = "blackbox";
          packages = with pkgs; [
            cargo
            rustc
            cargo-edit
            cargo-watch
            cargo-nextest
            cargo-tarpaulin
            cargo-audit
            cargo-deny
            cargo-flamegraph
            cargo-make
            pkg-config
            openssl
            sqlite
            protobuf
            nixpkgs-fmt
            treefmt
            typos
            just
            mdbook
          ];
          env = {
            RUSTUP_HOME = "$HOME/.rustup";
            CARGO_HOME = "$HOME/.cargo";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl pkgs.sqlite pkgs.stdenv.cc.cc.lib ];
          };
          shellHook = ''
            echo "Blackbox — Lunar Tear Rust Rewrite"
            echo ""
            echo "Commands:"
            echo "  cargo build --workspace"
            echo "  cargo test --workspace"
            echo "  cargo check --workspace"
          '';
        };
        formatter = pkgs.nixpkgs-fmt;
      };
    };
}
