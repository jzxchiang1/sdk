{ system ? builtins.currentSystem
, pkgs ? import ./nix { inherit system isMaster labels; }
, src ? builtins.fetchGit ./.
, releaseVersion ? "latest"
, RustSec-advisory-db ? pkgs.sources.advisory-db
, isMaster ? true
, labels ? {}
}:
rec {
  dfx = import ./dfx.nix { inherit pkgs assets; };

  e2e-tests = import ./e2e { inherit pkgs dfx system; };
  e2e-tests-ic-ref = import ./e2e { inherit pkgs dfx system; use_ic_ref = true; };

  check-binaries = import ./check-binaries.nix { inherit pkgs; };

  assets = import ./assets.nix { inherit pkgs distributed-canisters; };

  distributed-canisters = import ./distributed-canisters.nix { inherit pkgs; };

  inherit (pkgs) nix-fmt nix-fmt-check;

  install = import ./public { inherit pkgs src; };

  # This is to make sure CI evaluates shell derivations, builds their
  # dependencies and populates the hydra cache with them. We also use this in
  # `shell.nix` in the root to provide an environment which is the composition
  # of all the shells here.
  shells = {
    rust-workspace = dfx.shell;
  };

  licenses = {
    dfx = pkgs.lib.runtime.runtimeLicensesReport dfx.build;
  };
}
