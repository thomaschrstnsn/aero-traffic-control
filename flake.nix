{
  description = "A CLI tool for intelligent window management using AeroSpace";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin";
      pkgs = nixpkgs.legacyPackages.${system};
      cargoToml = pkgs.lib.importTOML ./Cargo.toml;
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;

        src = pkgs.lib.fileset.toSource {
          root = ./.;
          fileset = pkgs.lib.fileset.unions [ ./Cargo.toml ./Cargo.lock ./src ];
        };
        cargoLock.lockFile = ./Cargo.lock;

        meta = with pkgs.lib; {
          description = "A CLI tool for intelligent window management using AeroSpace";
          license = with licenses; [ mit asl20 ];
          platforms = platforms.darwin;
          mainProgram = "aero-traffic-control";
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ self.packages.${system}.default ];
        packages = with pkgs; [ clippy rustfmt rust-analyzer ];
      };
    };
}
