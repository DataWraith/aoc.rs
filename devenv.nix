{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.bacon
    pkgs.cargo-nextest
    pkgs.cargo-generate
    pkgs.just
    pkgs.openssl
    pkgs.pkg-config
    pkgs.typos
  ];

  # https://devenv.sh/integrations/difftastic/
  difftastic.enable = true;

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.rust.channel = "nightly";
  languages.rust.mold.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
