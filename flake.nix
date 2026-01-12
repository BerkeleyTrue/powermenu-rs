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
  }: let
    winitRuntimeLibs = pkgs:
      with pkgs; [
        wayland
        libxkbcommon
      ];
    mkPackage = pkgs: let
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      runtimeLibs = winitRuntimeLibs pkgs;
    in
      pkgs.rustPlatform.buildRustPackage {
        pname = manifest.name;
        version = manifest.version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        # buildtime
        nativeBuildInputs = with pkgs; [
          pkg-config
          wrapGAppsHook4
          gobject-introspection
        ];

        # runtime
        buildInputs =
          (with pkgs; [
            gtk4
            gtk4-layer-shell
            libadwaita
            gdk-pixbuf
            # needed for icons
            librsvg
            # needed for video streaming
            gst_all_1.gstreamer
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
            gst_all_1.gst-libav
          ])
          ++ runtimeLibs;
        preFixup = ''
          gappsWrapperArgs+=(
            --prefix GST_PLUGIN_SYSTEM_PATH_1_0 : "${pkgs.lib.makeSearchPath "lib/gstreamer-1.0" (with pkgs.gst_all_1; [gstreamer gst-plugins-base gst-plugins-good gst-libav])}"
            --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath runtimeLibs}"
            --set GTK_A11Y none
          )
        '';
        meta = with pkgs.lib; {
          description = "Powermenu in rust and relm4";
          license = licenses.mit;
          mainProgram = manifest.name;
        };
      };
  in
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
          overlays = [];
        };
        runtimeLibs = winitRuntimeLibs pkgs;
      in {
        packages.default = mkPackage pkgs;
        formatter.default = pkgs.alejandra;
        devShells.default = pkgs.mkShell {
          name = "${manifest.name}";

          nativeBuildInputs =
            (with pkgs; [
              cargo
              cargo-generate
              cargo-watch
              clippy
              rustc
              rustfmt

              openssl
            ])
            ++ runtimeLibs;

          LD_LIBRARY_PATH =
            lib.makeLibraryPath (
              (with pkgs; [
                gcc
                libiconv
                llvmPackages.llvm
              ])
              ++ runtimeLibs
            );
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
      flake = {
        overlays.default = final: prev: let
          manifest = (prev.lib.importTOML ./Cargo.toml).package;
        in {
          ${manifest.name} = mkPackage final;
        };
      };
    };
}
