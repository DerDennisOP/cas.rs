{
  description = "cas.rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
    };

    rustEnv = with pkgs.rustPackages; [
      clippy
    ];
  in
  {
    devShells.default = with pkgs; mkShell {
      buildInputs = [
        stdenv.cc.cc.lib
        pam
      ];

      packages = [
        cargo
        cargo-nextest
        rustc
        rustfmt
        rustEnv
      ];

      RUST_BACKTRACE = 1;
    };
  });
}
