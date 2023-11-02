{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kak-tabs";
  version = "0.1.6";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
