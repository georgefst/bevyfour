{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with (pkgs); [
            alsa-lib
            cargo
            pkg-config
            rustc
            udev
            wayland
          ];
          LD_LIBRARY_PATH = pkgs.lib.concatStringsSep ":" [
            "${pkgs.wayland}/lib"
            "${pkgs.libxkbcommon}/lib"
            "${pkgs.vulkan-loader}/lib"
            "${pkgs.vulkan-validation-layers}/lib"
          ];
        };
      }
    );
}
