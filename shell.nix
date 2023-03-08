{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ cmake vulkan-loader libxkbcommon wayland fontconfig ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
