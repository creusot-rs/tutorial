{
  inputs = {
    nixpkgs.follows = "creusot/nixpkgs";
    flake-utils.follows = "creusot/flake-utils";

    creusot.follows = "creusot-ide/creusot";
    creusot-ide.url = "github:creusot-rs/creusot-ide";
  };

  outputs = {
    self,
    creusot,
    creusot-ide,
    flake-utils,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      os-release = pkgs.writeTextFile {
        destination = "/etc/os-release";
        name = "os-release";
        text = "ID=nixos";
      };

      rust-analyzer-code = pkgs.fetchurl {
        name = "rust-analyzer-2026-01-05";

        url = "https://github.com/rust-lang/rust-analyzer/releases/download/2026-01-05/rust-analyzer-linux-arm64.vsix";
        hash = "sha256-iQvsNwNQgq9t/o/4CHa1ai+RB1N4oAXcrUdO2TiykfQ=";

        downloadToTemp = true;
        recursiveHash = true;
        postFetch = "mkdir $out && cp $downloadedFile $out/rust-analyzer.vsix";
      };
    in {
      packages.default = pkgs.dockerTools.buildLayeredImage {
        name = "creusot-dev";

        contents =
          [rust-analyzer-code os-release]
          ++ (with pkgs; [busybox patchelf])
          ++ (with pkgs.dockerTools; [caCertificates usrBinEnv])
          ++ (with creusot-ide.outputs.packages.${system}; [code lsp])
          ++ (with creusot.outputs.devShells.${system}.default; [rust-analyzer rust-src])
          ++ (with creusot.outputs.packages.${system}; [default]);

        config = {
          Cmd = ["/bin/sh"];
          Env = [
            "HOME=/root"
            "USER=root"
            "VSCODE_SERVER_CUSTOM_GLIBC_LINKER=${pkgs.glibc.outPath}/lib/ld-linux-x86-64.so.2"
            "VSCODE_SERVER_CUSTOM_GLIBC_PATH=${pkgs.glibc.outPath}/lib:${pkgs.libgcc.lib.outPath}/lib"
            "VSCODE_SERVER_PATCHELF_PATH=${pkgs.patchelf.outPath}/bin/patchelf"
          ];
        };

        enableFakechroot = true;

        fakeRootCommands = ''
          echo "root:x:0:" > /etc/group
          echo "nobody:x:65534:" >> /etc/group

          echo "root:x:0:0:root user:/var/empty:/bin/sh" > /etc/passwd
          echo "nobody:x:65534:65534:nobody:/var/empty:/bin/sh" >> /etc/passwd
        '';
      };

      formatter = pkgs.alejandra;
    });
}
