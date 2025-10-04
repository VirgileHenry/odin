{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.mold
    pkgs.python313
    pkgs.python313Packages.requests
  ];
  RUST_BACKTRACE=1;
  TMPDIR="/tmp";
}
