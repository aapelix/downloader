use chksum::sha1;
use std::path::PathBuf;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum VerifyStatus {
    /// The file has not been verified
    #[default]
    NotVerified,
    /// The file failed the verification process.
    Failed,
    /// The file passed the verification process.
    Ok,
}

impl std::fmt::Display for VerifyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::NotVerified => "Not Verified",
                Self::Failed => "FAILED",
                Self::Ok => "Ok",
            }
        )
    }
}

pub fn verify_file(expected_hash: &str, path: PathBuf) -> VerifyStatus {
    // Try to compute the SHA-1 hash of the file
    match sha1::chksum(&path) {
        Ok(digest) => {
            // Compare with the expected hash
            if digest.to_hex_lowercase() == expected_hash.to_lowercase() {
                VerifyStatus::Ok
            } else {
                VerifyStatus::Failed
            }
        }
        Err(_) => VerifyStatus::Failed,
    }
}
