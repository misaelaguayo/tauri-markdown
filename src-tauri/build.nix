{
  bun,
  cairo,
  cargo,
  dbus,
  fetchFromGitHub,
  gdk-pixbuf,
  glib,
  gtk3,
  libdrm,
  librsvg,
  openssl,
  pandoc,
  perl,
  pkg-config,
  rustPlatform,
  stdenv,
  vite,
  webkitgtk,
}:

rustPlatform.buildRustPackage rec {
  pname = "tauri-markdown";
  version = "v0.1.0";

  CARGO_NET_GIT_FETCH_WITH_CLI = "true";
  CARGO_HTTP_CHECK_REVOKE = "true";

  # src = fetchFromGitHub {
  #   owner = "misaelaguayo";
  #   repo = "tauri-markdown";
  #   rev = "v0.1.0";
  #   sha256 = "sha256-UjNueI2aIUF3s6AKnefhOhJGhf7lBKSeT0Fagw3AO78=";
  # };

  src = builtins.path { path = ./.; name = "build"; };

  cargoHash = "sha256-NXiZ4aZ16Z3JjElkWA7QLgHgidZfOKYpgSDinUXrjpY=";

  nativeBuildInputs = [
    perl
    pkg-config
  ];

  buildInputs = [ 
    cairo
    cargo
    dbus
    gdk-pixbuf
    glib
    gtk3
    libdrm
    librsvg
    openssl
    webkitgtk
    ];

  preCheck = ''
    HOME=$TMPDIR
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/tauri-markdown-ts $out/bin
  '';
}
