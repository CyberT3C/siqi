# siqi.nix
{ lib
, rustPlatform
}:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {

    name = manifest.name;
    src = lib.cleanSource ./.;
  
    cargoLock.lockFile = ./Cargo.lock;

    meta = with lib; {
      description = "changeme";
      license = licenses.mit;
      homepage = "https://github.com/CyberT3C/siqi";
      mainProgram = "siqi";
    };
}
