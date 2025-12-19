#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version { pub major: u32, pub minor: u32, pub patch: u32 }
impl Version {
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.trim().split('.');
        Some(Self {
            major: parts.next()?.parse().ok()?,
            minor: parts.next().unwrap_or("0").parse().ok()?,
            patch: parts.next().unwrap_or("0").parse().ok()?,
        })
    }
    pub fn compatible(&self, other: &Version) -> bool { self.major == other.major }
    pub fn to_string(&self) -> String { format!("{}.{}.{}", self.major, self.minor, self.patch) }
}
