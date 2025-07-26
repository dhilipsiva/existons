{
  description = "Rust dev shell for Existons";

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

        myBuildInputs = with pkgs; [
          # Rust
          rustc
          cargo
          clippy
          rustfmt
          rust-analyzer

          # X11 + OpenGL graphics deps
          libGL
          libGLU
          libdrm
          xorg.libX11
          xorg.libXrandr
          xorg.libXcursor
          xorg.libXi
          mesa
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = myBuildInputs;

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath myBuildInputs}"
            echo "✅ Existons shell ready — OpenGL and X11 libraries loaded"
          '';

          RUST_BACKTRACE = "1";
        };
      });
}

