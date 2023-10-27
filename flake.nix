{
  description = "qk";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
        pkgs = import nixpkgs { inherit system; };
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          inherit (cargoToml.package) name version;
          src = nixpkgs.lib.cleanSource ./.;
          doCheck = true;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
