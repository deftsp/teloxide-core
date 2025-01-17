use serde::{Deserialize, Serialize};

/// This object represents a file ready to be downloaded.
///
/// The file can be downloaded via the [`Bot::download_file(file_path, dst)`]
/// method. It is guaranteed that the path from [`GetFile`] will be valid for at
/// least 1 hour. When the path expires, a new one can be requested by calling
/// [`GetFile`].
///
/// [The official docs](https://core.telegram.org/bots/api#file).
///
/// [`GetFile`]: crate::payloads::GetFile
/// [`Bot::download_file(file_path, dst)`]: crate::net::Download::download_file
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// File size, if known.
    ///
    /// **Note:** in the Telegram Bot API this field is optional, however it was
    /// errourneusly marked as required in Teloxide. To workaround this issue,
    /// when `file_size` is not present, it is deserialized as [`u32::MAX`].
    #[serde(default = "default_file_size")]
    pub file_size: u32,

    /// File path. Use [`Bot::download_file(file_path, dst)`] to get the file.
    ///
    /// **Note:** in the Telegram Bot API this field is optional, however it was
    /// errourneusly marked as required in Teloxide. To workaround this issue,
    /// when `file_path` is not present, it is deserialized as an empty string.
    ///
    /// [`Bot::download_file(file_path, dst)`]: crate::net::Download::download_file
    #[serde(default)]
    pub file_path: String,
}

const fn default_file_size() -> u32 {
    u32::MAX
}

#[cfg(test)]
mod tests {
    use crate::types::File;

    #[test]
    fn no_file_size() {
        let json =
            r#"{"file_id":"FILE_ID","file_unique_id":"FILE_UNIQUE_ID","file_path":"FILE_PATH"}"#;
        let file: File = serde_json::from_str(json).unwrap();

        assert_eq!(
            file,
            File {
                file_id: "FILE_ID".to_owned(),
                file_unique_id: "FILE_UNIQUE_ID".to_owned(),
                file_size: u32::MAX,
                file_path: "FILE_PATH".to_owned(),
            }
        );
    }

    #[test]
    fn no_file_path() {
        let json = r#"{"file_id":"FILE_ID","file_unique_id":"FILE_UNIQUE_ID","file_size":42}"#;
        let file: File = serde_json::from_str(json).unwrap();

        assert_eq!(
            file,
            File {
                file_id: "FILE_ID".to_owned(),
                file_unique_id: "FILE_UNIQUE_ID".to_owned(),
                file_size: 42,
                file_path: "".to_owned(),
            }
        );
    }
}
