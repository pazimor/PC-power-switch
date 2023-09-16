{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
    buildInputs = with pkgs; [ cargo rustc rustup clang llvmPackages.bintools ];
}