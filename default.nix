# default.nix
let
  pkgs = import <nixpkgs> { };
in
{
  siqi = pkgs.callPackage ./siqi.nix { };
}
