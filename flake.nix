{
  description = "Packages for Wayland interaction";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-26.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      ftplugin = pkgs.stdenvNoCC.mkDerivation {
        name = "nvim-rs-ftplugin";
        src =
          pkgs.writeTextDir "ftplugin/rust.lua" # lua
            ''
              local root_files = {
                '.git',
                'Cargo.toml',
              }

              vim.lsp.start {
                name = 'rust-analyzer',
                cmd = { '${pkgs.rust-analyzer}/bin/rust-analyzer' },
                root_dir = vim.fs.dirname(vim.fs.find(root_files, { upward = true })[1]),
                -- capabilities = require('user.lsp').make_client_capabilities(),
              }
            '';

        installPhase = ''
          mkdir -p $out
          cp -r $src/* $out/
        '';
      };
    in
    {

      packages.${system}.default = pkgs.mkShell {

        buildInputs = with pkgs; [
          wayland
          wayland-protocols
          libxkbcommon
          pkg-config
          vulkan-loader
          vulkan-validation-layers

          cargo
          rustc
          rustfmt
          clippy
        ];

        shellHook = ''
            export NVIM_RTP_EXTRA=${ftplugin}
            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath [
                pkgs.wayland
                pkgs.libxkbcommon
                pkgs.vulkan-loader
              ]
            }

          exec /usr/bin/env fish 2> /dev/null
        '';

      };

    };
}
