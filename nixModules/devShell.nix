{
  config,
  pkgs,
  libraries,
  rustToolchainFile,
  moreBuildInputs ? [ ],
  ...
}:
{
  inputsFrom = [
    config.treefmt.build.devShell
  ];

  ###
  ## Build Tools
  ###

  buildInputs =
    with pkgs;
    let
      tools = [
        just
      ];
      rustInputs = [
        clang
        mold
        pkg-config
        (rust-bin.fromRustupToolchainFile rustToolchainFile)
      ];
    in
    tools ++ rustInputs ++ libraries ++ moreBuildInputs;

  ###
  ## Static Linking
  ###

  RUSTFLAGS =
    # Use mold as linker.
    [
      "-C linker=clang"
      "-C link-arg=-fuse-ld=mold"
    ]
    # Add precompiled library to rustc search path
    ++ builtins.map (a: ''-L ${a}/lib'') libraries;

  ###
  ## Dynamic Linking
  ###

  # TODO: we could be more strict about what goes in here.
  #
  # Some libraries need to be in the dynamic linker path (for "dlopen").
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;

  ###
  ## Rust Bindgen Setup
  ###

  # So bindgen can find libclang.so
  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_16.libclang.lib ];
  # Add headers to bindgen search path
  BINDGEN_EXTRA_CLANG_ARGS =
    let
      libClang = pkgs.llvmPackages_16.libclang.lib;
      libClangVersion = pkgs.llvmPackages_16.libclang.version;
    in
    [
      ''-I"${libClang}/lib/clang/${libClangVersion}/include"''
    ];
}
