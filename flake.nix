{
  description = "A Rust project with flake-parts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    
    flake-parts.url = "github:hercules-ci/flake-parts";
    
    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      # Define the systems for which to build
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

      # Import the rust-flake module
      imports = [ ];

      perSystem = { pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
          ];
          config = { };
        };
        # Define a development shell
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Add Rust toolchain
            (fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            rust-analyzer-nightly
          ];
          # Optionally set environment variables
          # RUST_LOG = "debug";
        };
      };
    };
}