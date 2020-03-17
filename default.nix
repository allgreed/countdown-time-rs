with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc 
    cargo
  ];
  buildInputs = [
  ];

  RUST_BACKTRACE = 1;
}
