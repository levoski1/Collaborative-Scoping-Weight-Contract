pragma solidity ^0.8.20;

interface IReputationManager {
    error InsufficientReputation(address user, uint256 have, uint256 need);
    error DecayAlreadyApplied(uint256 currentEpoch);

    event ReputationMinted(address indexed user, uint256 amount);
    event ReputationBurned(address indexed user, uint256 amount);
    event ReputationDecayed(address indexed user, uint256 newBalance);
    event EpochAdvanced(uint256 indexed epoch);

    function balanceOf(address user) external view returns (uint256);
    function mintReputation(address user, uint256 amount) external;
    function burnReputation(address user, uint256 amount) external;
    function consumeReputation(address user, uint256 amount) external returns (bool);
    function decayAll() external;
    function reputationToWeight(uint256 reputation) external pure returns (uint256);
}
