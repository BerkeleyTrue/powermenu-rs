{
  description = "Powermenu in rust and relm4";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    flake-parts,
    nixpkgs,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [];
      systems = ["x86_64-linux"];
      perSystem = {
        system,
        lib,
        ...
      }: let
        # Manifest via Cargo.toml
        manifest = (lib.importTOML ./Cargo.toml).package;
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
          ];
        };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook4
            gobject-introspection
          ];

          buildInputs = with pkgs; [
            gtk4
            gtk4-layer-shell
            libadwaita
            gdk-pixbuf
            librsvg
          ];

          meta = with lib; {
            description = "Powermenu in rust and relm4";
            license = licenses.mit;
            mainProgram = manifest.name;
          };
        };

        formatter.default = pkgs.alejandra;
        devShells.default = pkgs.mkShell {
          name = "${manifest.name}";

          nativeBuildInputs = with pkgs; [
            cargo
            cargo-generate
            cargo-watch
            clippy
            rustc
            rustfmt

            openssl

            gtk4
            gtk4-layer-shell
            meson
            ninja
            parted
            gettext
            appstream
            pkg-config
            gdk-pixbuf
            libadwaita
            librsvg
            gnome-desktop
            wrapGAppsHook4
            desktop-file-utils
            gobject-introspection
            rustPlatform.bindgenHook
            gst_all_1.gstreamer
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
            gst_all_1.gst-libav
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [gcc libiconv llvmPackages.llvm]);
          LIBCLANG_PATH = lib.makeLibraryPath [pkgs.libclang];
          NIX_LDFLAGS = "-L${pkgs.libiconv}/lib";

          # Set Environment Variables
          RUST_BACKTRACE = "full";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules";
          GTK_A11Y = "none";

          shellHook = ''
            function menu () {
              echo
              echo -e "\033[1;34m>==> ️  '$name' shell\n\033[0m"
              just --list
              echo
              echo "(Run 'just --list' to display this menu again)"
              echo
            }

            export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
            menu
            just --list
          '';
        };
      };
      flake = {};
    };
}
