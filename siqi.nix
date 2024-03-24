# siqi.nix
{ lib
, stdenv
, fetchFromGitHub
}:

stdenv.mkDerivation {
  name = "siqi";

  src = fetchFromGitHub{
    repo = "siqi";
  };
}
