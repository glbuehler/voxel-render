{
  description = "Packages for Wayland interaction";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {

    packages.${system}.default = pkgs.mkShell {

      buildInputs = with pkgs; [
        wayland
        wayland-protocols
        libxkbcommon
        pkg-config
        vulkan-loader
        vulkan-validation-layers
      ];

      shellHook = ''
        # exec ${pkgs.fish}/bin/fish
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
        ]}
      '';

    };

  };
}
