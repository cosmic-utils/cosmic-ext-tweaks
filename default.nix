{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.callPackage (
  {
    lib,
    rustPlatform,
    just,
    openssl,
    libxkbcommon,
    libGL,
    libX11,
    libXcursor,
    libXi,
    libXrandr,
    vulkan-loader,
    wayland,
    pkg-config,
    makeWrapper,
    stdenv,
    cosmic-comp,
    cosmic-icons,
  }:
  let
    pname = "cosmic-ext-tweaks";
    version = "0.2.6";

    buildInputs = [
      openssl
      libGL
      libxkbcommon
      vulkan-loader
      wayland
      libX11
      libXcursor
      libXi
      libXrandr
    ];
  in
  rustPlatform.buildRustPackage {
    inherit pname version buildInputs;

    src = builtins.path {
      name = "${pname}-source";
      path = ./.;
    };

    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "accesskit-0.22.0" = "sha256-6TWK4HXc2a4XSf13/nksZoihMqZurQ7tBGY8+2DaPuY=";
        "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
        "clipboard_macos-0.1.0" = "sha256-WO3JFbE+6ESRAfkxrnEFeZyGuhUHLOKOVHcGQyHwoK0=";
        "cosmic-client-toolkit-0.2.0" = "sha256-5BEUK/+RwSk6X3/4NRY91Ef1b/vZ0nk0oObN0Y1ylz0=";
        "cosmic-config-1.0.0" = "sha256-QgZqsfHpg7+JwV7IT1Fa/nBanm2JVI4ohN0yBFGt6/E=";
        "cosmic-ext-config-templates-3.0.0" = "sha256-2g+clhM4tVMADJ9I/odR+VBFOVWtqroez9iG5vkq6pI=";
        "cosmic-freedesktop-icons-0.4.0" = "sha256-tPriTi5L0mFMHjo5xpF5cmKGHqlX3WUO7EZAgVdBpS4=";
        "cosmic-panel-config-1.11.0" = "sha256-qBCQZOCgWjRVmu8KqSME2eoRfW78tL8bfWP7NAd5v/A=";
        "cosmic-settings-config-1.11.0" = "sha256-gFd6uEZw3SWGjuf59K1ixs/wxS5zeWby/OprECHAN2M=";
        "cosmic-settings-daemon-zbus-1.0.0" = "sha256-suu8Wmzqe9EGo+LrOQ7N0iVydT/X8qPsU4grqTWD+Tw=";
        "cryoglyph-0.1.0" = "sha256-10JUHl1ktbqLaReuiU3HPa4r2KvsoryyJoF3BFoge3U=";
        "dpi-0.1.2" = "sha256-/H+D2Szeaufte6gbGQelOcvU4JhncJ9k9XTFr0q7PTw=";
        "smithay-clipboard-0.8.0" = "sha256-GojAFRbhJcP0Rpr+v9WOivgW9x38PZdeBWTbMhkDB3A=";
        "softbuffer-0.4.1" = "sha256-9Ret/nfieBFl4yJ9TddyWsSuS7sI4QAza/TZrxYMb+I=";
      };
    };

    nativeBuildInputs = [
      just
      pkg-config
      makeWrapper
    ];

    dontUseJustBuild = true;
    dontUseJustCheck = true;

    justFlags = [
      "--set"
      "prefix"
      (placeholder "out")
      "--set"
      "bin-src"
      "target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/cosmic-ext-tweaks"
    ];

    postInstall = ''
      wrapProgram $out/bin/cosmic-ext-tweaks \
        --suffix XDG_DATA_DIRS : "${cosmic-icons}/share" \
        --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath buildInputs}
    '';

    meta = {
      changelog = "https://github.com/cosmic-utils/tweaks/releases/tag/${version}";
      description = "Tweaking tool for the COSMIC Desktop Environment";
      homepage = "https://github.com/cosmic-utils/tweaks";
      license = lib.licenses.gpl3Only;
      maintainers = with lib.maintainers; [ HeitorAugustoLN ];
      mainProgram = "cosmic-ext-tweaks";
      inherit (cosmic-comp.meta) platforms;
    };
  }
) { }
