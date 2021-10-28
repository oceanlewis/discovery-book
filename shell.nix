let
  sources = import ./nix/sources.nix;
  rustChannel = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  rust = rustChannel.rust.override {
    targets = [ "thumbv7em-none-eabihf" ];
    extensions = [
      "clippy-preview"
      "rust-src"
      "rustfmt-preview"
      "rust-analysis"
    ];
  };
in
pkgs.mkShell {
  buildInputs = [ rust ] ++ (with pkgs; [
    rust-analyzer
    cargo-edit
    cargo-watch
    cargo-udeps
    cargo-binutils
    gcc-arm-embedded
    openocd
    minicom
  ]);
}
