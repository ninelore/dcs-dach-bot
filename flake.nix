{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      forSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      overlays = [ (import rust-overlay) ];
    in
    {
      checks = forSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system overlays;
            config.allowUnsupportedSystem = true;
          };
          check =
            deps: cmd:
            pkgs.stdenv.mkDerivation {
              name = builtins.concatStringsSep "-" (pkgs.lib.sublist 0 2 (pkgs.lib.splitString " " cmd));
              dontBuild = true;
              src = ./.;
              cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
              doCheck = true;
              nativeBuildInputs = deps ++ [
                pkgs.cargo
                pkgs.rustPlatform.cargoSetupHook
              ];
              checkPhase = "mkdir $out; " + cmd;
              installPhase = "";
            };
        in
        {
          cargo-test = check [ ] "cargo test --verbose --all-features";
          clippy = check [ pkgs.clippy ] "cargo clippy --all-features -- -D warnings";
          formatted = check [ pkgs.rustfmt ] "cargo fmt --all -- --check";
          tarpaulin = check [
            pkgs.cargo-tarpaulin
          ] "cargo tarpaulin --verbose --all-features --out Xml --output-dir $out";
        }
      );

      # For codecov file
      packages = forSystems (system: {
        tarpaulin = self.checks.${system}.tarpaulin;
      });

      devShells = forSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system overlays;
            config.allowUnsupportedSystem = true;
          };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              (pkgs.rust-bin.fromRustupToolchain {
                channel = "stable";
                components = [
                  "rustfmt"
                  "rust-src"
                  "clippy"
                ];
                targets = [ "wasm32-unknown-unknown" ];
                profile = "minimal";
              })
              cargo-deny
              cargo-edit
              cargo-tarpaulin
              openssl
              pkg-config
              rust-analyzer
            ];
          };
        }
      );
    };
}
