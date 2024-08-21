{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux"];
    eachSystem = nixpkgs.lib.genAttrs systems;

    overlays = [(import rust-overlay)];

    pkgsFor = eachSystem (
      system:
        import nixpkgs {inherit system overlays;}
    );
  in {
    devShells = eachSystem (
      system: let
        pkgs = pkgsFor.${system};
      in
        with pkgs; {
          default = mkShell {
            packages = [
              (rust-bin.stable.latest.default.override {
                extensions = ["rust-src" "rust-analyzer"];
              })
            ];
          };
        }
    );

    packages = eachSystem (
      system: let
        pkgs = pkgsFor.${system};

        tt-rs = pkgs.rustPlatform.buildRustPackage {
          pname = "tt-rs";
          version = "0.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      in {
        inherit tt-rs;
        default = tt-rs;
      }
    );
  };
}
