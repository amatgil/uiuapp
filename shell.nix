{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:
let
  rustPlatformD = pkgs.makeRustPlatform {
    cargo = pkgs.rust-bin.stable.latest.default.override { targets = [ "wasm32-unknown-unknown" ]; };
    rustc = pkgs.rust-bin.stable.latest.default.override { targets = [ "wasm32-unknown-unknown" ]; };
  };

  packages = with pkgs; [
    rust-analyzer
    rustfmt
    mold
    #rust-bin.stable.latest.default
    (rust-bin.stable.latest.default.override {
      targets = [ "wasm32-unknown-unknown" "aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android" ];
    })

    # For linux native
    pkg-config
    glib
    gdk-pixbuf
    cairo
    gtk3
    #webkit2gtk-sys
    libsoup
    nss
    nspr
    xorg.libxkbfile
    xorg.libXcursor
    libsForQt5.qt5ct
    qt5Full
    xorg.libxcb
    xorg.xcbutil
    xorg.xcbutilimage
    xorg.xcbutilkeysyms
    xorg.xcbutilwm
    xorg.xcbutilrenderutil
    libxkbcommon
    xorg.libX11
    xorg.libXext
    xorg.libXcursor
    xorg.libXinerama

    xcb-util-cursor
    xcb-proto
    openssl
    pkg-config
    cargo-ndk

#androidComposition = pkgs.androidenv.composeAndroidPackages {
#  includeNDK = true;
#  includeSDK = true;
#  platformToolsVersion = "34.0.5";
#  ndkVersion = "26.1.10909125";
#};



    (rustPlatformD.buildRustPackage rec {
      pname = "dioxus-cli";
      version = "0.6.3";

      src = fetchCrate {
        inherit pname version;
        hash =  "sha256-wuIJq+UN1q5qYW4TXivq93C9kZiPHwBW5Ty2Vpik2oY=";
      };

      cargoHash = "sha256-L9r/nJj0Rz41mg952dOgKxbDS5u4zGEjSA3EhUHfGIk=";

      nativeBuildInputs = [
        pkg-config
        cacert
      ];
      buildInputs = [ openssl ];

      OPENSSL_NO_VENDOR = 1;

      nativeCheckInputs = [ rustfmt ];

      passthru = {
        updateScript = nix-update-script { };
        tests.version = testers.testVersion { package = dioxus-cli; };
      };
    })




    (rustPlatform.buildRustPackage rec {
      pname = "wasm-bindgen-cli";
      version = "0.2.100";
      cargoHash = "sha256-qsO12332HSjWCVKtf1cUePWWb9IdYUmT+8OPj/XP2WE=";

      src = fetchCrate {
        inherit pname version;
        hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
      };

      buildInputs = [ openssl ];
      nativeBuildInputs = [ pkg-config ];
    })

    android-studio
    #androidenv.androidPkgs_9_0.androidsdk
    #(android-studio.withSdk (androidenv.composeAndroidPackages { includeNDK = true; }).androidsdk)
  ];
in
pkgs.mkShell {
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
