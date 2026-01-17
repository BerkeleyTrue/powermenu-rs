{
  description = "Powermenu in Iced-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    nixgl.url = "github:nix-community/nixGL";

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    flake-parts,
    nixpkgs,
    nixgl,
    ...
  }: let
    winitRuntimeLibs = pkgs:
      with pkgs; [
        wayland
        libxkbcommon

        vulkan-loader
        vulkan-headers
        libGL
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
          makeWrapper
        ];

        # runtime
        buildInputs = runtimeLibs;
        postFixup = ''
          wrapProgram $out/bin/${manifest.name} \
            --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath runtimeLibs}"
        '';
        meta = with pkgs.lib; {
          description = "Powermenu in rust and iced-rs";
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
          overlays = [
            nixgl.overlay
          ];
        };
        runtimeLibs = winitRuntimeLibs pkgs;
        atlasConfig = {
          input = "resources/redlotoo_dead-internet.mp4";
          output = "resources/redlotoo_dead-internet-atlas.png";
          # keep width under 4096px with 910px wide tiles
          columns = 4;
        };
        atlasScript = pkgs.writeShellApplication {
          name = "generate-atlas";
          runtimeInputs = with pkgs; [
            bc
            coreutils
            ffmpeg
            imagemagick
          ];
          text = ''
            set -euo pipefail

            input=${lib.escapeShellArg atlasConfig.input}
            output=${lib.escapeShellArg atlasConfig.output}
            columns=${toString atlasConfig.columns}
            target_fps=15

            if [ ! -f "$input" ]; then
              echo "Expected to find $input relative to the project root." >&2
              exit 1
            fi

            # Get video duration and calculate frame count at target FPS
            duration=$(ffprobe -v error -show_entries format=duration -of csv=p=0 "$input")
            # Calculate: duration * fps, rounded to nearest integer
            frame_count=$(printf "%.0f" "$(echo "$duration * $target_fps" | bc -l)")

            echo "Video duration: $duration seconds"
            echo "Target FPS: $target_fps"
            echo "Will extract $frame_count frames"

            frames_dir="$(mktemp -d)"
            trap 'rm -rf "$frames_dir"' EXIT

            # Extract frames at target fps, 512px tall, nearest neighbor scaling (ok for pixel art)
            ffmpeg -hide_banner -loglevel error -i "$input" \
              -vf "fps=$target_fps,scale=-2:512:flags=neighbor" \
              -frames:v "$frame_count" \
              -vsync 0 \
              "$frames_dir/frame_%04d.png"

            mkdir -p "$(dirname "$output")"
            montage "$frames_dir"/frame_*.png \
              -background none -alpha set \
              -tile "$columns"x -geometry +0+0 \
              "$output"

            echo "Atlas saved to $output with $frame_count frames"
            echo "Update FRAMES constant in src/dead_internet.rs to: $frame_count"
          '';
        };
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
            ++ [pkgs.nixgl.nixGLMesa atlasScript]
            ++ runtimeLibs;

          WINIT_UNIX_BACKEND = "wayland";
          LD_LIBRARY_PATH = lib.makeLibraryPath (
            (with pkgs; [
              gcc
              libiconv
              llvmPackages.llvm
            ])
            ++ runtimeLibs
          );
          LIBCLANG_PATH = lib.makeLibraryPath [pkgs.libclang];
          NIX_LDFLAGS = "-L${pkgs.libiconv}/lib";
          NIXGL = "${pkgs.nixgl.nixGLMesa}/bin/nixGLMesa";

          RUST_BACKTRACE = "full";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          shellHook = ''
            function menu () {
              echo
              echo -e "\033[1;34m>==> ️  '$name' shell\n\033[0m"
              just --list
              echo
              echo "(Run 'just --list' to display this menu again)"
              echo
            }

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
