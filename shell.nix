{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.mold
  ];
  RUST_BACKTRACE=1;
  TMPDIR="/tmp";
}
