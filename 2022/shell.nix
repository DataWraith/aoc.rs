{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    bacon
    cargo-generate
    cargo-nextest
    just
    rustup
    stdenv
  ];
}
