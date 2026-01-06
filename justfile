default:
  just --list

[group('build')]
build:
  cargo build
  notify -a rust -i text-rust "Build Complete" "Powermenu build complete"

[group('build')]
run:
  cargo run
