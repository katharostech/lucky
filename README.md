# Lucky - A charming framework for Juju

[![Build Status Badge][build_badge_img]][build_badge_lnk] [![Snap Build Status Badge][snap_build_img]][snap_build_lnk] ![Lines of Code Badge][lines_of_code_img]

[build_badge_img]: https://cloud.drone.io/api/badges/katharostech/lucky/status.svg
[build_badge_lnk]: https://cloud.drone.io/katharostech/lucky
[snap_build_img]: https://build.snapcraft.io/badge/katharostech/lucky.svg
[snap_build_lnk]: https://build.snapcraft.io/user/katharostech/lucky
[lines_of_code_img]: https://tokei.rs/b1/github/katharostech/lucky?category=code

This is the home of a work-in-progress charm writing framework for [Juju]. The framework will seek to provide a well-documented and easy-to-use experience for writing charms that run Docker containers. While currently we are focusing on Docker containers, as development progresses, this framework could very well end up being useful for writing non-Docker charms as well.

The original discussion that started this effort can be found [here][discussion] on the [Juju forums][forum].

The documentation is hosted at [katharostech.github.io/lucky].

[juju]: https://jaas.ai
[discussion]: https://discourse.jujucharms.com/t/is-the-reactive-framework-making-juju-slow-my-experiences-with-juju-so-far/2282/9?u=zicklag
[forum]: https://discourse.jujucharms.com/
[katharostech.github.io/lucky]: https://katharostech.github.io/lucky

## Installation

Lucky 0.1.0-alpha.0 is out! Lucky is now ready to be tested by other users. A getting started guide will be released soon, but for now we have the CLI [documentation][katharostech.github.io/lucky] and you can get started by downloading the build for your platform from the [releases] page.

If you need any help, don't hesitate to reach out on the [Juju forum][juju_forum].

[releases]: https://github.com/katharostech/lucky/releases

## Bugs, Features, and Questions

If you have any bug reports or feature requests you can create a [Taiga issue][taiga_issue] and we'll see what we can do to help.

For questions or general discussion there is also a [Lucky category][category] on the [Juju forum][juju_forum].

[category]: https://discourse.jujucharms.com/c/related-software/lucky
[juju_forum]: https://discourse.jujucharms.com
[taiga_issue]: https://tree.taiga.io/project/zicklag-lucky/issues