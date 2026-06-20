project(
  'audio_output_switcher',
  'rust',
  version: '1.0.0',
  license: 'Unlicense',
  meson_version: '>= 1.7.0',
  default_options: ['rust_std=2021'],
)

cargo = find_program('cargo')
cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'target']

cargo_release = custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    console: true,
    output: 'audio_output_switcher',
    install: true,
    install_dir: join_paths(get_option('prefix'), get_option('bindir')),
    command: [
        cargo,
        'build',
        '--release', cargo_options,
        '&&',
        'cp', 'target' / 'release' / meson.project_name(), '@OUTPUT@' 
    ],
)
