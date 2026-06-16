{
  description = "A flake using Oxalica's rust-overlay wrapped with bevy-flake.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    bevy-flake = {
      url = "github:swagtop/bevy-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    bevy-flake,
    rust-overlay,
    ...
  }:
    bevy-flake.lib.mkFlake {
      perSystem = {
        pkgs,
        system,
        packages,
        formatter,
        ...
      }: {
        inherit packages formatter;

        devShells.default = pkgs.mkShell {
          name = "bevy-flake-rust-overlay";
          packages = [
            packages.rust-toolchain.develop
            packages.dioxus-cli.develop
            packages.bevy-cli.develop
          ];
          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
          '';
        };
      };

      config = {
        pkgs,
        system,
        ...
      }: {
        src = builtins.path {
          name = "src";
          path = ./.;

          # Ignore files that aren't needed in compilation of Bevy project.
          filter = path: type:
            !(builtins.elem (baseNameOf path) [
              "flake.lock"
              "flake.nix"
            ]);
        };

        rustToolchain = targets: let
          baseToolchain =
            if builtins.pathExists ./rust-toolchain.toml
            then pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
            else pkgs.rust-bin.stable.latest.default;
        in
          baseToolchain.override {
            inherit targets;
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };

        withPkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            (final: prev: {
              wasm-bindgen-cli = prev.rustPlatform.buildRustPackage rec {
                pname = "wasm-bindgen-cli";
                version = "0.2.121";
                hash = "sha256-ZOMgFNOcGkO66Jz/Z83eoIu+DIzo3Z/vq6Z5g6BDY/w=";
                cargoHash = "sha256-DPdCDPTAPBrbqLUqnCwQu1dePs9lGg85JCJOCIr9qjU=";
                src = prev.fetchCrate {
                  inherit pname version hash;
                };
              };
            })
          ];
          config = {
            allowUnfree = true;
            microsoftVisualStudioLicenseAccepted = true;
          };
        };
      };
    };
}
