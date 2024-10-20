{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
	rustVersion = pkgs.rust-bin.stable.latest.default;
	rustPlatform = pkgs.makeRustPlatform {
	  cargo = rustVersion;
	  rustc = rustVersion;
	};
	cargotoml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
	thispkg = rustPlatform.buildRustPackage {
	  pname = cargotoml.package.name;
	  version = cargotoml.package.version;
	  src = ./.;
	  cargoLock.lockFile = ./Cargo.lock;
	};
      in {
        defaultPackage = thispkg;
        devShell = pkgs.mkShell {
	  buildInputs = [
	    (rustVersion.override { extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ]; })
	  ];
	};
      });
}
