{
  description = "nfswalkdir dev shell and nix derivation";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
    };
  };
  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust.default;
        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          nativeBuildInputs = [ pkgs.rustPlatform.bindgenHook ];
          buildInputs = [
            pkgs.libnfs
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          installCargoArtifactsMode = "use-zstd";
        });

        nfs = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = cargoArtifacts;
          doCheck = false;
        });

        nfswalk = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

      in
      with pkgs;
      {
        packages = {
          default = nfswalkdir;
        };
        apps.default = flake-utils.lib.mkApp {
          drv = nfswalkdir;
        };

        devShell = mkShell {
          inputsFrom = [ nfs ];

          buildInputs = [
            rust-analyzer
            strace
            cargo-audit
          ];

          nativeBuildInputs = [
            (
              rust.default.override
                {
                  extensions = [
                    "rust-src"
                  ];
                }
            )
          ];

          shellHook = ''
            rustc --version
          '';

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        };
      }
    );
}
