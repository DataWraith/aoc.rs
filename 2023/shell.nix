{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    bacon
    cargo-generate
    cargo-nextest
    clang
    just
    rustup
    stdenv
  ];
}
