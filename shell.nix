let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { config.allowUnfree = true; };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    pkg-config
    openssl
    ngrok
    dhall
    dhall-json
    binaryen
  ];
}
