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
              cargo-tauri
              rust-analyzer
              openssl
              pkg-config
            ];
          };
        }
      );
    };
}
