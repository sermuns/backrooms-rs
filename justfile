run:
    RUST_LOG=info,wgpu_hal=off cargo run

watch:
    watchexec --restart --exts rs -- just run
