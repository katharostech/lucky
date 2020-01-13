def main(ctx):
    # Config used by pipeline functions
    config = {}

    # The drone context
    config["ctx"] = ctx

    # The version of rust to use for building Lucky
    config["rust_version"] = "nightly-2019-11-24"
    
    pipelines = []

    # Skip all pipelines on feature branches to prevent building twice:
    # one time for PR and one time for feature branch push.
    if ctx.build.branch.startswith("feature/"):
        return []

    # Add Linux pipelines
    pipelines.extend(linux_pipelines(config))

    # Add Windows pipelines
    pipelines.extend(windows_pipelines(config))

    # Add MacOS pipelines
    pipelines.extend(macos_pipelines(config))

    return pipelines

def linux_pipelines(config):
    # ( arch name, rust target, download name ) tuple pairs
    archs = [
        ("amd64", "x86_64-unknown-linux-musl", "x86_64"),
        # TODO: Figure out how to make the ARM builds work
        # ("arm64", "aarch64-unknown-linux-musl"),
        # ("arm", "arm-unknown-linux-musleabi")
    ]

    # Return pipelines for each architecture
    return [linux_pipeline(config, arch) for arch in archs]

# Build a linux pipeline for a specific arch
def linux_pipeline(config, arch):
    pipeline = {
        "kind": "pipeline",
        "name": "linux-" + (arch[0]),
    }

    # Pipeline steps
    steps = []

    # Build Lucky
    steps.append({
        "name": "build",
        "image": "clux/muslrust:" + config["rust_version"],
        "commands": [
            # Install rust target for platform
            "rustup target add " + arch[1],
            # Compile Lucky
            "cargo build --release --target " + arch[1],
            # Create tarball
            "mkdir -p build",
            "mv target/" + arch[1] + "/release/lucky build",
            "cd build/",
            "tar -czf lucky-linux-" + arch[2] + ".tgz lucky"
        ]
    })

    # Build Documentation ( only on amd64 just so that we only do it once )
    if arch[0] == "amd64":
        # Generate CLI documentation
        steps.append({
            "name": "generate-cli-doc",
            "image": "clux/muslrust:" + config["rust_version"],
            "depends_on": ["build"], # Wait for build because of cargo target lock
            "commands": [
                "cargo run --release --features doc-gen docs/book/src/"
            ]
        })

        # Build markdown book
        steps.append({
            "name": "build-book",
            "image": "hrektts/mdbook:latest",
            "depends_on": ["generate-cli-doc"],
            "commands": [
                "cd docs/book",
                "mdbook build"
            ]
        })

        # Publish documentation if pushed to master
        if config["ctx"].build.branch == "master":
            steps.append({
                "name": "publish-book",
                "image": "plugins/gh-pages",
                "depends_on": ["build-book"],
                "settings": {
                    "pages_directory": "docs/book/build",
                    "username": {
                        "from_secret": "github_username",
                    },
                    "password": {
                        "from_secret": "github_access_key"
                    }
                }
            })

    # Pre-release steps
    if config["ctx"].build.ref == "refs/tags/pre-release":
        steps.append({
            "name": "publish-github-pre-release",
            "image": "plugins/github-release",
            "depends_on": ["build"],
            "settings": {
                "title": "pre-release",
                "prerelease": True,
                "api_key": {
                    "from_secret": "github_access_key",
                },
                "files": [
                    "build/lucky-linux-" + arch[2] + ".tgz"
                ]
            }
        })

    # TODO: Add release step for v* tags

    # Set the pipeline steps
    pipeline["steps"] = steps

    return pipeline

def windows_pipelines(config):
    # Create the primary ( and only, so far ) Windows pipeline
    pipeline = {
        "kind": "pipeline",
        "name": "windows",
    }

    # Pipeline steps
    steps = []

    # Build step: Compile Lucky for Windows
    steps.append({
        "name": "build",
        "image": "rust:latest",
        "commands": [
            # Install MingW
            "apt-get update",
            "apt-get install -y gcc-mingw-w64-x86-64 zip",
            # Install the version of Rust that we need
            "rustup default " + config["rust_version"],
            "rustup target install x86_64-pc-windows-gnu",
            # Patch Some MingW Libs. See https://github.com/rust-lang/rust/issues/47048#issuecomment-474118132
            "cp -f /usr/x86_64-w64-mingw32/lib/dllcrt2.o /usr/local/rustup/toolchains/nightly-2019-11-24-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/dllcrt2.o",
            "cp -f /usr/x86_64-w64-mingw32/lib/crt2.o /usr/local/rustup/toolchains/nightly-2019-11-24-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/crt2.o",
            # Configure Cargo to use the MingW linker
            "mkdir -p .cargo",
            """printf '[target.x86_64-pc-windows-gnu]\nlinker = "x86_64-w64-mingw32-gcc"' >> .cargo/config""",
            # Compile Lucky for Windows
            "cargo build --target x86_64-pc-windows-gnu --release --no-default-features --features default_devkit",
            # Zip up the Lucky executable
            "mkdir -p build",
            "mv target/x86_64-pc-windows-gnu/release/lucky.exe build",
            "cd build/",
            "zip -r lucky-windows-x86_64.zip lucky.exe"
        ]
    })

    # Choco: Create the Chocolatey Package
    steps.append({
        "name": "build-choco",
        "depends_on": ["build"],
        "image": "linuturk/mono-choco",
        "commands": [
            "cd chocolatey",
            # Replace the Lucky version placeholder in the nuspec file
            """sed -i "s/{{version}}/0.1.0-pre-release-${DRONE_BUILD_NUMBER}/" lucky.nuspec""",
            # Create the package
            "choco pack"
        ]
    })

    # Pre-release steps
    if config["ctx"].build.ref == "refs/tags/pre-release":
        # Publish Chocolatey pre-release
        steps.append({
            "name": "publish-choco-pre-release",
            "depends_on": ["build-choco"],
            "image": "linuturk/mono-choco",
            "environment": {
                "API_KEY": {
                    "from_secret": "chocolatey_api_key"
                }
            },
            "commands": [
                "cd chocolatey",
                # Push to chocolatey.org
                """choco push --api-key "$${API_KEY}" --source https://push.chocolatey.org/""",
            ]
        })

        # Publish pre-release to GitHub releases
        steps.append({
            "name": "publish-github-pre-release",
            "image": "plugins/github-release",
            "depends_on": ["build"],
            "settings": {
                "title": "pre-release",
                "prerelease": True,
                "api_key": {
                    "from_secret": "github_access_key",
                },
                "files": [
                    "build/lucky-windows-x86_64.zip"
                ]
            }
        })

    # TODO: Add release step for v* tags

    # Set pipeline steps
    pipeline["steps"] = steps

    # Return our only Windows pipeline
    return [pipeline]

def macos_pipelines(config):
     # Create the primary ( and only, so far ) MacOS pipeline
    pipeline = {
        "kind": "pipeline",
        "name": "macos",
    }

    # Pipeline steps
    steps = []

    # Build step
    steps.append({
        "name": "build",
        "image": "katharostech/rust-osxcross:rust-latest_v0.1.0",
        "commands": [
            # Add OSXCross tools to the PATH
            "PATH=\"$PATH:/build/osxcross/target/bin\"",
            # Install our target version of Rust
            "rustup default " + config["rust_version"],
            "rustup target install x86_64-apple-darwin",
            # Configure cargo to use the Mac linker and libraries
            "export CC=x86_64-apple-darwin15-clang",
            "mkdir -p /drone/src/.cargo",
            """printf '[target.x86_64-apple-darwin]\\nlinker = "x86_64-apple-darwin15-clang"' >> /drone/src/.cargo/config""",
            # Compile Lucky
            "cargo build --target x86_64-apple-darwin --release --no-default-features --features default_devkit",
            # Create a tarball
            "mkdir -p build",
            "mv target/x86_64-apple-darwin/release/lucky build",
            "cd build/",
            "tar -czf lucky-mac-x86_64.tgz lucky",
            # Calculate the sha256sum ( used for the homebrew cask )
            "sha256sum lucky-mac-x86_64.tgz | awk -F ' ' '{print $1}' > sha256.txt"
        ]
    })

    # Pre-release steps
    if config["ctx"].build.ref == "refs/tags/pre-release":
        # Publish pre-release to GitHub releases
        steps.append({
            "name": "publish-github-pre-release",
            "image": "plugins/github-release",
            "depends_on": ["build"],
            "prerelease": True,
            "settings": {
                "title": "pre-release",
                "api_key": {
                    "from_secret": "github_access_key",
                },
                "files": [
                    "build/lucky-mac-x86_64.tgz"
                ]
            }
        })

        # Update the Lucky pre-release Homebrew cask
        steps.append({
            "name": "publish-homebrew-pre-release",
            "image": "alpine/git",
            "depends_on": ["publish-github-pre-release"],
            "environment": {
                "USER": {
                    "from_secret": "github_username",
                },
                "PASSWORD": {
                    "from_secret": "github_access_key"
                }
            },
            "commands": [
                # Configure Git
                'git config --global user.email "zicklag@katharostech.com"',
                'git config --global user.name "Zicklag"',
                # Clone our Homebrew tap
                "git clone https://$${USER}:$${PASSWORD}@github.com/katharostech/homebrew-tap.git",
                # Substitue the sha256 inside the cask file with the newly calculated sum
                """sed -i "/\\\\w*sha256/s/\\\\'[a-z0-9]*\\\\'/\\\\'$(cat build/sha256.txt)\\\\'/" homebrew-tap/Casks/lucky-pre-release.rb""",
                "cd homebrew-tap",
                # Commit and push the update
                "git add .",
                "git commit -m 'Update Lucky Pre-Release'",
                "git push"
            ]
        })

    # TODO: Add release step for v* tags

    # Set pipeline steps
    pipeline["steps"] = steps

    # Return our only MacOS pipeline
    return [pipeline]
