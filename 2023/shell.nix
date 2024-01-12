{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    bacon
    cargo-generate
    cargo-nextest
    just
    rustup
    stdenv
    z3
  ];

  Z3_SYS_Z3_HEADER = "${pkgs.z3.dev}/include/z3.h";
  LIBCLANG_PATH="${pkgs.llvmPackages_14.libclang.lib}/lib";
}
