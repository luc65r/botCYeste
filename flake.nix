{
  description = "A very basic flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, mozilla }:
    flake-utils.lib.eachDefaultSystem (system: let
      rustOverlay = final: prev: let
        rustChannel = prev.rustChannelOf {
          channel = "1.52.1";
          sha256 = "6eRkXrYqS/7BYlx7OBw44/phnDKN6l9IZjSt3eh78ZQ=";
        };
      in {
        inherit rustChannel;
        rustc = rustChannel.rust;
        cargo = rustChannel.rust;
      };

      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import "${mozilla}/rust-overlay.nix")
          rustOverlay
        ];
      };
    in {
      defaultPackage = with pkgs; (makeRustPlatform {
        rustc = rustChannel.rust;
        cargo = rustChannel.rust;
      }).buildRustPackage {
        pname = "botCYeste";
        version = "0.2.0";
        src = self;
        cargoSha256 = "Q8azSW9Ps4vie26FSHYetLxGkyiwXU6VZvpFzpqmMSs=";
        buildInputs = [
          sqlite
        ];
      };

      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustChannel.rust
          diesel-cli
          sqlite
        ];

        # rust-analyzer is broken with 1.50
        shellHook = ''
          PATH="${pkgs.lib.makeBinPath [pkgs.rust-analyzer]}:$PATH"
        '';

        DATABASE_URL = "db.sqlite3";
      };
    });
}
