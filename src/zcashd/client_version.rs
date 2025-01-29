use anyhow::Result;

use crate::Parseable;

#[derive(Clone, PartialEq)]
pub struct ClientVersion {
    version: u32,
    major: u32,
    minor: u32,
    revision: u32,
    build: u32,
}

impl ClientVersion {
    /// Parses a combined version integer into its components.
    ///
    /// Per zcashd's `clientversion.h`, the version is a 32-bit integer
    ///
    /// The integer is expected to be in the format:
    /// `major * 1_000_000 + minor * 10_000 + revision * 100 + build`
    ///
    /// # Arguments
    ///
    /// * `version` - The combined version integer.
    ///
    /// # Returns
    ///
    /// A `ClientVersion` struct with separated version components.
    pub fn from_integer(version: u32) -> Self {
        let major = version / 1_000_000;
        let remainder = version % 1_000_000;

        let minor = remainder / 10_000;
        let remainder = remainder % 10_000;

        let revision = remainder / 100;
        let build = remainder % 100;

        ClientVersion {
            version,
            major,
            minor,
            revision,
            build,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn build(&self) -> u32 {
        self.build
    }
}

impl Parseable for ClientVersion {
    fn parse_type() -> &'static str {
        "ClientVersion"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let version = parser.parse_u32()?;
        Ok(ClientVersion::from_integer(version))
    }
}

impl std::fmt::Display for ClientVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.revision, self.build
        )
    }
}

impl std::fmt::Debug for ClientVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
