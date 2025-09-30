{ lib
, stdenv
, crane
, pkg-config
, pkgs
, openssl
}:

let
  craneLib = crane.mkLib pkgs;
  pname = "github-to-jenkins-webhook";

  # Clean source to avoid unnecessary rebuilds
  src = craneLib.cleanCargoSource (../.);

  # Common arguments shared between both derivations
  commonArgs = {
    inherit pname src;
    nativeBuildInputs = [ pkg-config ];
    buildInputs = [ openssl ];
  };

  # Build dependencies separately for better caching
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
# Build the actual application
craneLib.buildPackage (commonArgs // {
  inherit cargoArtifacts;

  # Using a datestamped "unstable" is fine inside a repo.
  version = "unstable-2025-09-23";

  # Install examples for easy reference (optional).
  postInstall = ''
    mkdir -p $out/share/${pname}/examples
    cp -r examples/* $out/share/${pname}/examples/ 2>/dev/null || true
  '';

  meta = {
    description = ''
      Securely forward webhooks to Jenkins in your private infrastructure.
    '';
    homepage = "https://github.com/LoganBarnett/${pname}";
    license = lib.licenses.mit;
    mainProgram = pname;
    platforms = lib.platforms.unix;
  };
})
