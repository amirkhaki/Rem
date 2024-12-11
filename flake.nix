{
  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        my-rust = (pkgs.rust-bin.stable.latest.default.override {
              extensions = ["rust-analyzer" "rust-docs" "rust-src"];
            });
            in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            my-rust
            (writeShellScriptBin "rust-doc" ''
  xdg-open '${my-rust}/share/doc/rust/html/index.html'
'')
            gdb
          ];

        };
      }
    );
}
