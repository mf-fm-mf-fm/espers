{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ vulkan-loader libxkbcommon wayland cmake fontconfig ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
