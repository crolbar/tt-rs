{
  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in with pkgs;
      {
        defaultPackage = (makeRustPlatform {
          inherit cargo rustc;
        }).buildRustPackage {
            cargoLock.lockFile = ./Cargo.lock;
            version = "0.1";
            pname = "tt-rs";
            src = ./.;
        };
      }
    );
}
