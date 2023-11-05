{
  description = "qk";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec
      {
        packages = {
          qk = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) version;
            pname = cargoToml.package.name;
            src = nixpkgs.lib.cleanSource ./.;
            doCheck = true;
            cargoLock.lockFile = ./Cargo.lock;
          };
          default = packages.qk;
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
          ];
        };
      }
    );
}
