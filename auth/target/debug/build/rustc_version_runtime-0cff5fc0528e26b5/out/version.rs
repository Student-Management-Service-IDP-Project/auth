
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 71,
                        patch: 0,
                        pre: vec![semver::Identifier::AlphaNumeric("nightly".to_owned()), ],
                        build: vec![],
                    },
                    host: "aarch64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.71.0-nightly (5cdb7886a 2023-04-15)".to_owned(),
                    commit_hash: Some("5cdb7886a5ece816864fab177f0c266ad4dd5358".to_owned()),
                    commit_date: Some("2023-04-15".to_owned()),
                    build_date: None,
                    channel: Channel::Nightly,
                }
            }
            