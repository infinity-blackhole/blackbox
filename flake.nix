{
  inputs = {
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devlib = {
      url = "github:shikanime-studio/devlib";
      inputs = {
        devenv.follows = "devenv";
        flake-parts.follows = "flake-parts";
        git-hooks.follows = "git-hooks";
        nixpkgs.follows = "nixpkgs";
        treefmt-nix.follows = "treefmt-nix";
      };
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [
      "https://cachix.cachix.org"
      "https://devenv.cachix.org"
      "https://shikanime.cachix.org"
      "https://shikanime-studio.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cachix.cachix.org-1:eWNHQldwUO7G2VkjpnjDbWwy4KQ/HNxht7H4SSoMckM="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "shikanime.cachix.org-1:OrpjVTH6RzYf2R97IqcTWdLRejF6+XbpFNNZJxKG8Ts="
      "shikanime-studio.cachix.org-1:KxV6aDFU81wzoR9u6pF1uq0dQbUuKbodOSP8/EJHXO0="
    ];
  };

  outputs =
    inputs@{
      devenv,
      devlib,
      flake-parts,
      git-hooks,
      treefmt-nix,
      pkgs,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        devenv.flakeModule
        devlib.flakeModule
        git-hooks.flakeModule
        treefmt-nix.flakeModule
      ];
      perSystem = _: {
        devenv.shells.default = {
          imports = [
            devlib.devenvModules.git
            devlib.devenvModules.nix
            devlib.devenvModules.shell
            devlib.devenvModules.shikanime-studio
          ];
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
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl pkgs.sqlite pkgs.stdenv.cc.cc.lib ];
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
        };
      };
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
    };
}
