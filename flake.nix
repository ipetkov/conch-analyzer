{
  description = "A flake for building the conch-analyzer app";

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
     let
       pkgs = import nixpkgs {
         inherit system;
         overlays = [
           (import inputs.rust-overlay)
         ];
       };
       rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
         targets = [
           "wasm32-unknown-unknown"
         ];
       };
     in rec {
       devShell = pkgs.mkShell {
         buildInputs = [];
         nativeBuildInputs = with pkgs; [
           nodejs
           nodePackages.npm
           rust-toolchain
           wabt
           wasm-bindgen-cli
           wasm-pack
         ];
       };
     }
  );
}
