{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [
          cargo
          git
          rustc
        ];
        buildInputs = with pkgs; [
          nixd
          rust-analyzer
          rustPackages.clippy
          rustToolchain
        ];
        naersk' = pkgs.callPackage naersk { };
      in
      with pkgs;
      {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            exec fish
          '';
        };
      }
    );
}
