{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    cargo-bloat
    cargo-expand
    cargo-flamegraph
    cargo-geiger
    cmake
    fontconfig
    libxkbcommon
    vulkan-loader
    wayland
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
