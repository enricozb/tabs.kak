{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kak-tabs";
  version = "0.2.0.beta";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
