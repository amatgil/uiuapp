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


    (rustPlatformD.buildRustPackage rec {
      pname = "dioxus-cli";
      version = "0.6.3";

      src = fetchCrate {
        inherit pname version;
        hash =  "sha256-wuIJq+UN1q5qYW4TXivq93C9kZiPHwBW5Ty2Vpik2oY=";
      };

      cargoHash = "sha256-LnNLsU8bbbVIUltBbhLYkRTcFMPwkrLjHf19U44BHy4=";

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
      cargoHash = "sha256-tD0OY2PounRqsRiFh8Js5nyknQ809ZcHMvCOLrvYHRE=";

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
