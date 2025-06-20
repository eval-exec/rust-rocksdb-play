{
  description = "GCC 14 Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default =
        pkgs.mkShell.override {
          stdenv = pkgs.gcc14Stdenv;
        } {
          buildInputs = [ pkgs.gcc14 ];
        };
    };
}

