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

          buildInputs = [pkgs.makeWrapper];

          cargoHash = "sha256-GDFzSK2P5V+F5NBt+qq0aq4kQJNqD8z793RD73ESYrg=";

          postInstall = ''
            mkdir -p $out/.config/tt-rs
            cp conf/* $out/.config/tt-rs
            mv "$out/bin/tt-rs" "$out/bin/.tt-rs-wrapped"
            makeWrapper "$out/bin/.tt-rs-wrapped" "$out/bin/tt-rs" \
                --set HOME "$out/"
          '';
        };
      in {
        inherit tt-rs;
        default = tt-rs;
      }
    );
  };
}
