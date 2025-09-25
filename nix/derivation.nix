{ lib
, stdenv
, rustPlatform
, pkg-config
, openssl
}:

rustPlatform.buildRustPackage (let
  pname = "github-to-jenkins-webhook";
in {
  inherit pname;
  # Using a datestamped "unstable" is fine inside a repo.
  version = "unstable-2025-09-23";

  # Use the repository root as the source when packaging in-repo.
  src = lib.cleanSource (../.);

  # Use Cargo.lock from the repo so we don’t need a vendor hash.
  # If you have git-based crates, add outputHashes here as needed.
  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = { };
  };

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [
    openssl
  ];

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
