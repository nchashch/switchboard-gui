let
  pkgs = import <nixpkgs> {  };
in
  with pkgs;
  mkShell {
    buildInputs = [
      pkgconfig
      openssl
      sass
      glib
      cairo
      pango
      atk
      gdk-pixbuf
      libsoup
      gtk3
      webkitgtk
      librsvg
      patchelf
      trunk
      dbus
      nodePackages_latest.pnpm

      qrencode
      qt5.qtbase
      qt5.qttools
      protobuf
    ];
    WEBKIT_DISABLE_COMPOSITING_MODE = 1;
    QT_PLUGIN_PATH="${qt5.qtbase}/${qt5.qtbase.qtPluginPrefix}";
}
