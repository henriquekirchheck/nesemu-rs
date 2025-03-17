{
  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
          ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

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

        nativeBuildInputs = with pkgs; [
          wayland.dev
          pkg-config
          toolchain
        ];
      in
      {
        packages = {
          default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;
            inherit buildInputs nativeBuildInputs;
          };
        };

        devShell = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          packages = [ pkgs.gdb ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
