{
  pkgs ? import <nixpkgs> { },
}:
let
  clangPkg = pkgs.llvmPackages_16; # or _15, _17, etc. depending on your need
in
pkgs.mkShell {
  buildInputs = [
    clangPkg.clang
  ];

  # required by bindgen
  shellHook = ''
    export LIBCLANG_PATH="${clangPkg.libclang.lib}/lib"
  '';
}
