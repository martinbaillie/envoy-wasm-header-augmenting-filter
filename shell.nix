let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { config.allowUnfree = true; };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    direnv
    rust
    pkg-config
    openssl
    ngrok
    dhall
    dhall-json
    binaryen
  ];
  shellHook = ''
    set -o vi
    eval "$(direnv hook $SHELL)"
    direnv allow
    cd .
  '';
}
