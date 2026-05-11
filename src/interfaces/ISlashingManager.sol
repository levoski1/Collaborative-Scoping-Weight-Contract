pragma solidity ^0.8.20;

interface ISlashingManager {
    struct SlashRecord {
        address maintainer;
        string issueUrl;
        uint256 penalty;
        uint256 timestamp;
        bool applied;
    }

    error NotAuthorized();
    error DuplicateSlash();
    error NoChangeDetected();
    error PenaltyExceedsMax();

    event MaintainerSlashed(
        address indexed maintainer,
        string indexed issueUrl,
        uint256 penalty,
        string reason
    );
    event SlashSettled(address indexed maintainer, uint256 amount);
    event MaxPenaltyUpdated(uint256 oldMax, uint256 newMax);

    function slash(
        address maintainer,
        string calldata issueUrl,
        uint256 previousPoints,
        uint256 newPoints,
        string calldata reason
    ) external returns (uint256 penalty);

    function getSlashRecord(
        address maintainer,
        string calldata issueUrl
    ) external view returns (SlashRecord memory);

    function totalSlashed(address maintainer) external view returns (uint256);
}
