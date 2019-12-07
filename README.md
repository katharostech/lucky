# Lucky - A charming framework for Juju

[![Build Status][bb]][bl] [![Snap Status][ssi]][ssl] [![lucky on snapcraft][sb]][sl] ![Lines of code][lc]

[bb]: https://cloud.drone.io/api/badges/katharostech/lucky/status.svg
[bl]: https://cloud.drone.io/katharostech/lucky
[ssi]: https://build.snapcraft.io/badge/katharostech/lucky.svg
[ssl]: https://build.snapcraft.io/user/katharostech/lucky
[lc]: https://tokei.rs/b1/github/katharostech/lucky?category=code
[sb]: https://snapcraft.io//lucky/badge.svg
[sl]: https://snapcraft.io/lucky

This is the home of a work-in-progress charm writing framework for [Juju]. The framework will seek to provide a well-documented and easy-to-use experience for writing charms that run Docker containers. While currently we are focusing on Docker containers, as development progresses, this framework could very well end up being useful for writing non-Docker charms as well.

The original discussion that started this effort can be found [here][discussion] on the [Juju forums][forums].

The documentation is hosted at [katharostech.github.io/lucky].

[juju]: https://jaas.ai
[discussion]: https://discourse.jujucharms.com/t/is-the-reactive-framework-making-juju-slow-my-experiences-with-juju-so-far/2282/9?u=zicklag
[forums]: https://discourse.jujucharms.com/
[katharostech.github.io/lucky]: https://katharostech.github.io/lucky

## Installation

Currently there isn't much to try yet, but you can test the latest builds of Lucky by downloading the build for your platform from the [releases] page.

### Chocolatey

You can install Lucky with the [Chocolatey] package manager for Windows. Until we add the package to the community repo you have to download the package from the [releases] page. You can then install it from the commandline.

    choco install .\lucky.0.1.0-pre-release.nupkg

### Snap

We have a [snap] which can be installed on supported linux distros from the commandline:

    sudo snap install --edge --devmode lucky


[releases]: https://github.com/katharostech/lucky/releases
[snap]: https://snapcraft.io/lucky
[chocolatey]: https://chocolatey.org/