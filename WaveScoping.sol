// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title WaveScoping — Decentralized backlog prioritization via weighted voting
contract WaveScoping {
    struct Issue {
        string githubIssueUrl;
        uint256 currentWeight;
        uint256 assignedPoints;
        bool exists;
    }

    address public owner;

    mapping(string => Issue) public registry;
    // voter => issueUrl => weight already cast
    mapping(address => mapping(string => uint256)) public votesBy;

    uint256 public constant MAX_WEIGHT_PER_VOTE = 100;

    event IssueRegistered(string indexed url);
    event Voted(address indexed voter, string indexed url, uint256 weight);

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    /// @notice Register a new issue for the current Wave
    function registerIssue(string calldata _url) external onlyOwner {
        require(!registry[_url].exists, "Already registered");
        registry[_url] = Issue(_url, 0, 0, true);
        emit IssueRegistered(_url);
    }

    /// @notice Signal priority by casting weight on an issue (one vote per address per issue)
    function voteOnIssue(string calldata _url, uint256 _weight) external {
        require(registry[_url].exists, "Issue not registered");
        require(_weight > 0 && _weight <= MAX_WEIGHT_PER_VOTE, "Invalid weight");
        require(votesBy[msg.sender][_url] == 0, "Already voted");

        votesBy[msg.sender][_url] = _weight;
        registry[_url].currentWeight += _weight;
        registry[_url].assignedPoints = _calculatePoints(registry[_url].currentWeight);

        emit Voted(msg.sender, _url, _weight);
    }

    /// @dev Linear scaling: 10 weight units = 1 point
    function _calculatePoints(uint256 _w) internal pure returns (uint256) {
        return _w / 10;
    }

    /// @notice Read the current weight and points for an issue
    function getIssue(string calldata _url) external view returns (uint256 weight, uint256 points) {
        Issue storage i = registry[_url];
        require(i.exists, "Issue not registered");
        return (i.currentWeight, i.assignedPoints);
    }
}
