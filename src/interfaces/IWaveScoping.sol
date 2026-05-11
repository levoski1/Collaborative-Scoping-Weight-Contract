pragma solidity ^0.8.20;

interface IWaveScoping {
    struct Issue {
        string githubIssueUrl;
        uint256 currentWeight;
        uint256 assignedPoints;
        bool exists;
        bool isEmergency;
        address assignedContributor;
        uint256 startedAtBlock;
    }

    struct Wave {
        uint256 id;
        string name;
        uint256 startBlock;
        uint256 endBlock;
        bool isActive;
        bool isFinalized;
    }

    error NotOwner();
    error AlreadyRegistered();
    error NotRegistered();
    error AlreadyVoted();
    error InvalidWeight();
    error WaveNotActive();
    error WaveAlreadyFinalized();
    error NotContributor();
    error PointsAlreadyAssigned();
    error EmergencyOnly();
    error SlashAlreadyApplied();
    error ZeroAddress();
    error NotEnoughReputation();

    event IssueRegistered(string indexed url, uint256 waveId);
    event Voted(address indexed voter, string indexed url, uint256 weight);
    event IssueFastTracked(string indexed url, address indexed maintainer);
    event ContributionStarted(string indexed url, address indexed contributor);
    event ReputationEarned(address indexed voter, uint256 amount);
    event SlashApplied(address indexed maintainer, uint256 penalty);
    event WaveCreated(uint256 indexed id, string name, uint256 endBlock);
    event WaveFinalized(uint256 indexed id);
    event PointsAdjusted(string indexed url, uint256 oldPoints, uint256 newPoints);
}
