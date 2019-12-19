# Lucky - A charming framework for Juju

[![Build Status Badge][build_badge_img]][build_badge_lnk] [![Snap Build Status Badge][snap_build_img]][snap_build_lnk] ![Lines of Code Badge][lines_of_code_img]

[build_badge_img]: https://cloud.drone.io/api/badges/katharostech/lucky/status.svg
[build_badge_lnk]: https://cloud.drone.io/katharostech/lucky
[snap_build_img]: https://build.snapcraft.io/badge/katharostech/lucky.svg
[snap_build_lnk]: https://build.snapcraft.io/user/katharostech/lucky
[lines_of_code_img]: https://tokei.rs/b1/github/katharostech/lucky?category=code

This is the home of a work-in-progress charm writing framework for [Juju]. The framework will seek to provide a well-documented and easy-to-use experience for writing charms that run Docker containers. While currently we are focusing on Docker containers, as development progresses, this framework could very well end up being useful for writing non-Docker charms as well.

The original discussion that started this effort can be found [here][discussion] on the [Juju forums][forums].

The documentation is hosted at [katharostech.github.io/lucky].

[juju]: https://jaas.ai
[discussion]: https://discourse.jujucharms.com/t/is-the-reactive-framework-making-juju-slow-my-experiences-with-juju-so-far/2282/9?u=zicklag
[forums]: https://discourse.jujucharms.com/
[katharostech.github.io/lucky]: https://katharostech.github.io/lucky

## Installation

Currently there isn't much to try yet, but you can test the latest builds of Lucky by downloading the build for your platform from the [releases] page.

We also support the following package managers.

[releases]: https://github.com/katharostech/lucky/releases

### Chocolatey ( Windows )

[![Chocolatey Package Badge][choco_pack_badge_img]][choco_pack_badge_lnk] ![Chocolatey Downloads Badge][choco_dl_badge]

You can install Lucky with the [Chocolatey] package manager for Windows:

    choco install lucky --pre

[chocolatey]: https://chocolatey.org/
[choco_pack_badge_img]: https://img.shields.io/chocolatey/vpre/lucky?label=Chocolatey
[choco_pack_badge_lnk]: https://chocolatey.org/packages/lucky
[choco_dl_badge]: https://img.shields.io/chocolatey/dt/lucky?label=Downloads

### Snap ( Linux )

[![Get it from the Snap Store](https://snapcraft.io/static/images/badges/en/snap-store-black.svg)](https://snapcraft.io/lucky)

We have a [snap] which can be installed on supported linux distros from the commandline:

    sudo snap install --edge --devmode lucky

[snap]: https://snapcraft.io/lucky

### Homebrew ( MacOS )

[![Homebrew Cask Badge][brew_cask_img]][brew_cask_link]

[brew_cask_img]: https://img.shields.io/badge/Homebrew%20Cask-pre--release-yellow?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAYAAAAfSC3RAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAAadEVYdFNvZnR3YXJlAFBhaW50Lk5FVCB2My41LjExR/NCNwAAAaJJREFUOE9jwAa4uLiUpk+ffr6hoWEniAsRxQFkZWV1a2tr57W0tOxdsGDB6/9A8PPnz/9AAz7k5eVNhyrDAKxZWVnbQIqxgRkzZryysrLyhapFAEFBQb/v379DlWEHq1ev/gpVjgIESktLj29cv/r/qVOn/j9//vz/p0+fwBpev379f87smf/9/PxyoWpRQWpqau3fCz3/n66P+H+ix/j/tga9/9vXLfy/Ydn0/2/uHPm/bNmyB52dnQeASoUhOqAgJSWl5u+lif//niz8/3uDAxgfmeD4f1u39//Lh9f89/DwKNu0adNzaWnpUKgWCEhOTsbQCMMHN878D1TCv2TJkiuKiorhEB1QANRYTZbGpKQkPBpn4daYmJhYhUvjw4u7/re3t689fPjwRzExsQCoFghISEio+nux///fYzkYGh9c2Pm/u7t7Y2ZmZj9QKTtEBxQANVb+e3ft/98Lff//HC36/+dQzv8/J2v+/7279v/BfbtBTuWDqEQDcnJyWo2NjfOB6fTcqlWrHgNTynNg3N2aNGnSVmdn51SoMihgYAAADftfvlht2OgAAAAASUVORK5CYII=
[brew_cask_link]: https://github.com/katharostech/homebrew-tap/blob/master/Casks/lucky-pre-release.rb

We have a [Homebrew] cask:

    brew cask install katharostech/tap/lucky-pre-release

[homebrew]: https://brew.sh/

## Bugs, Features, and Questions

If you have any bug reports or feature requests you can create a [Taiga issue][taiga_issue] and we'll see what we can do to help.

For questions or general discussion there is also a [Lucky category][category] on the [Juju forum][juju_forum].

[category]: https://discourse.jujucharms.com/c/related-software/lucky
[juju_forum]: https://discourse.jujucharms.com
[taiga_issue]: https://tree.taiga.io/project/zicklag-lucky/issues