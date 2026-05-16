use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NotOwner,
    AlreadyRegistered,
    NotRegistered,
    AlreadyVoted,
    InvalidWeight,
    WaveNotActive,
    WaveAlreadyFinalized,
    PointsAlreadyAssigned,
    NotContributor,
    EmergencyOnly,
    SlashAlreadyApplied,
    ZeroAddress,
    NotEnoughReputation(String, u64, u64),
    InsufficientReputation(String, u64, u64),
    NotAuthorized,
    DuplicateSlash,
    NoChangeDetected,
    PenaltyExceedsMax,
    AlreadyProposed,
    TimelockNotMet,
    AlreadyExecuted,
    NotProposer,
    EmptyReason,
    DecayAlreadyApplied(u64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotOwner => write!(f, "Not owner"),
            Error::AlreadyRegistered => write!(f, "Already registered"),
            Error::NotRegistered => write!(f, "Issue not registered"),
            Error::AlreadyVoted => write!(f, "Already voted"),
            Error::InvalidWeight => write!(f, "Invalid weight"),
            Error::WaveNotActive => write!(f, "Wave not active"),
            Error::WaveAlreadyFinalized => write!(f, "Wave already finalized"),
            Error::PointsAlreadyAssigned => write!(f, "Points already assigned"),
            Error::NotContributor => write!(f, "Not contributor"),
            Error::EmergencyOnly => write!(f, "Emergency only"),
            Error::SlashAlreadyApplied => write!(f, "Slash already applied"),
            Error::ZeroAddress => write!(f, "Zero address"),
            Error::NotEnoughReputation(user, have, need) => {
                write!(f, "Not enough reputation for {user}: have {have}, need {need}")
            }
            Error::InsufficientReputation(user, have, need) => {
                write!(f, "Insufficient reputation for {user}: have {have}, need {need}")
            }
            Error::NotAuthorized => write!(f, "Not authorized"),
            Error::DuplicateSlash => write!(f, "Duplicate slash"),
            Error::NoChangeDetected => write!(f, "No change detected"),
            Error::PenaltyExceedsMax => write!(f, "Penalty exceeds max"),
            Error::AlreadyProposed => write!(f, "Already proposed"),
            Error::TimelockNotMet => write!(f, "Timelock not met"),
            Error::AlreadyExecuted => write!(f, "Already executed"),
            Error::NotProposer => write!(f, "Not proposer"),
            Error::EmptyReason => write!(f, "Empty reason"),
            Error::DecayAlreadyApplied(epoch) => write!(f, "Decay already applied for epoch {epoch}"),
        }
    }
}
