# SPDX-FileCopyrightText: 2024 The JMP.boot Developers
#
# SPDX-License-Identifier: GPL-3.0-only

{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          pkgs = nixpkgs.legacyPackages.${system};
          target = "aarch64-unknown-linux-gnu";
          toolchain = with fenix.packages.${system}; fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-6OHS9jbR+waO+TWanPqYYK3acOC4QR+g0z39rvmnmYQ=";
          };
        in

        (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          CARGO_BUILD_TARGET = target;
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
            let
              inherit (pkgs.pkgsCross.aarch64-multiplatform.stdenv) cc;
            in
            "${cc}/bin/${cc.targetPrefix}cc";
        };
    });
}
