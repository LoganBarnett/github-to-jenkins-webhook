{
  description = "A secure GitHub to Jenkins webhook proxy.";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }@inputs: let
    systems = [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems f;
    overlays = [
      (import rust-overlay)
    ];
    pkgsFor = system: import nixpkgs { inherit overlays system; };
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

    devShells = forAllSystems (system: {
      default = (pkgsFor system).mkShell {
        buildInputs = (packages (pkgsFor system));
        shellHook = ''
        '';
      };
    });

    overlays.default = final: prev: {
      github-to-jenkins-webhook = final.callPackage ./nix/derivation.nix {};
    };

    packages = forAllSystems (system: {
      default = (pkgsFor system).callPackage ./nix/derivation.nix {};
    });

  };

}
