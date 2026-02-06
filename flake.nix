{
  description = "sukr - bespoke static site compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn (pkgsFor system) (toolchainFor system));
    pkgsFor = system:
      import nixpkgs {
        system = system;
      };
    toolchainFor = system:
      fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-vra6TkHITpwRyA5oBKAHSX0Mi6CBDNQD+ryPSpxFsfg=";
      };
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    devShells = forAllSystems (
      pkgs: toolchain: {
        default = pkgs.mkShell.override {stdenv = pkgs.clangStdenv;} {
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          packages =
            [
              toolchain
              pkgs.treefmt
              pkgs.shfmt
              pkgs.rust-analyzer
              pkgs.taplo
              pkgs.pkg-config
              pkgs.nixfmt
              pkgs.nodePackages.prettier
              pkgs.miniserve # Dev server for testing
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.apple-sdk
              pkgs.libiconv
            ];
        };
      }
    );
    packages = forAllSystems (pkgs: toolchain: rec {
      sukr = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;
        src = ./.;
        buildInputs = [toolchain];
        cargoHash = "sha256-mPm8Pe4W9TyDuuXLHWqA9DzbkTyR1kkfLZ3SmEt+dUc=";
      };
      default = sukr;
    });
  };
}
