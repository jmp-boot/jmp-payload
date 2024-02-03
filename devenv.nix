{ ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  devcontainer.enable = true;
}
