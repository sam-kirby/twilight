use serde::{Deserialize, Serialize};

/// Current gateway session utilisation status.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SessionStartLimit {
    /// Maximum number of session that may be started concurrently.
    pub max_concurrency: u64,
    /// Number of remaining sessions for a given time period.
    pub remaining: u64,
    /// When the remaining sessions resets back to the total.
    pub reset_after: u64,
    /// Total number of sessions that can be started within the given time
    /// period.
    pub total: u64,
}

#[cfg(test)]
mod tests {
    use super::SessionStartLimit;
    use serde_test::Token;

    #[test]
    fn test_connection_info() {
        let start = SessionStartLimit {
            max_concurrency: 16,
            remaining: 998,
            reset_after: 84686789,
            total: 1000,
        };

        serde_test::assert_tokens(
            &start,
            &[
                Token::Struct {
                    name: "SessionStartLimit",
                    len: 4,
                },
                Token::Str("max_concurrency"),
                Token::U64(16),
                Token::Str("remaining"),
                Token::U64(998),
                Token::Str("reset_after"),
                Token::U64(84686789),
                Token::Str("total"),
                Token::U64(1000),
                Token::StructEnd,
            ],
        );
    }
}
