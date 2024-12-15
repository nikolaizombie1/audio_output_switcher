{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [  vscode-extensions.vadimcn.vscode-lldb  vscode-extensions.vscodevim.vim vscode-extensions.rust-lang.rust-analyzer cargo rustc ];
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
