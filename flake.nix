{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        buildInputs = with pkgs; [
          xorg.libX11
          xorg.libXcursor
          xorg.libxcb
          xorg.libXi
          xorg.libXrandr
          libxkbcommon

          vulkan-loader
          wayland
        ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          inherit buildInputs;
        };
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            gdb
          ];
          inherit buildInputs;
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
