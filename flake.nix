{
  description = "PrusaLink OpenAPI client generation environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            rustc
            cargo

            # Python for preprocessing the OpenAPI document
            python3
            python3Packages.pyyaml

            # Useful for inspecting/validating YAML
            yq-go
            jq

            # General development tools
            git
            curl
          ];

          shellHook = ''
            echo "PrusaLink development environment"
            echo
            echo "Rust:       $(rustc --version)"
            echo "Cargo:      $(cargo --version)"
            echo "Progenitor: $(cargo progenitor --version 2>/dev/null || echo installed)"
            echo "Python:     $(python3 --version)"
            echo

            # Make sure Python can find the packages supplied by Nix.
            export PYTHONUNBUFFERED=1
          '';
        };
      });
}
