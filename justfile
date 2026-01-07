default:
  just --list

[group('build')]
build:
  cargo build && \
    notify -a rust -i text-rust "Build Complete" "Powermenu build complete" || \
    notify -a rust -i text-rust "Build Failed!" "Powermenu build failed" 

[group('build')]
run:
  cargo run

[group('build')]
watch:
  cargo watch -w src -w resources -x "run -- --no-focus --dryrun"
