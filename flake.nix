{
  description = "A secure GitHub to Jenkins webhook proxy.";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }@inputs: let
    packages = (pkgs: let
      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          # For rust-analyzer and others.  See
          # https://nixos.wiki/wiki/Rust#Shell.nix_example for some details.
          "rust-src"
          "rust-analyzer"
          "rustfmt-preview"
        ];
      };
    in [
      pkgs.cargo-sweep
      pkgs.clang
      pkgs.cargo
      # To help with finding openssl.
      pkgs.pkg-config
      rust
      pkgs.rustfmt
      pkgs.rustup
    ]);
  in {

    devShells.aarch64-darwin.default = let
      system = "aarch64-darwin";
      overlays = [
        (import rust-overlay)
      ];
      pkgs = import nixpkgs {
        inherit overlays system;
      };
    in pkgs.mkShell {
      buildInputs = (packages pkgs);
      shellHook = ''
      '';
    };

  };
}
