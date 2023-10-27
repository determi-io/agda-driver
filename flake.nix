{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # agda
    # NOTE: We cannot use the nixpkgs.agda because it has
    # ghc as runtime dependency, making the resulting closure
    # very large!
    agda =
    {
      type = "github";
      owner = "determi-io";
      repo = "agda-only-agda";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, agda }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          AGDA = "${agda.outputs.packages.x86_64-linux.default}/bin/agda";
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo rust-analyzer ];
          AGDA = "${pkgs.agda}";
        };
      }
    );
}
