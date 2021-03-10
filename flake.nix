{
  description = "dot-files with variational types";

  inputs = {
    naersk.url = "github:nmattia/naersk";
    nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";
    nixpkgs-mozilla.flake = false;
  };

  outputs = { self, nixpkgs, nixpkgs-mozilla, naersk }
  :
    let forAllSystems = f: nixpkgs.lib.genAttrs
        [ "x86_64-linux" ] (system: f system);
        nixpkgsFor = forAllSystems (system:
          import nixpkgs {
            inherit system;
            overlays = [ (import nixpkgs-mozilla) ];
          }
        );
    in
      rec {

        defaultPackage = forAllSystems (
          system: let
            pkgs = nixpkgsFor.${system};
            naersk_ = pkgs.callPackage naersk {
              rustc = (
                pkgs.rustChannelOf {
                  date = "2021-03-09";
                  channel = "nightly";
                  sha256 = "sha256-UeQjaWNCLf9ng34rAkojEiCZizYz1XCWsDHZusj+00U=";
                }
              ).rust;
            };
          in
            naersk_.buildPackage {
              src = ./.;
            }
        );

        devShell = forAllSystems (
          system:
            let
              pkgs = nixpkgsFor.${system};
            in
            pkgs.mkShell {
              buildInputs = with pkgs; [ pkg-config gtk3 ];
            }
        );

      };
}
