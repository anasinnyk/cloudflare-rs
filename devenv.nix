{
  pkgs,
  ...
}:

{
  packages = with pkgs; [
    git
    lldb
    vscode-extensions.vadimcn.vscode-lldb
  ];

  env = {
    CODELLDB_PATH = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/adapter/codelldb";
    LIBLLDB_PATH = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/lldb/lib/liblldb.dylib";
  };

  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
