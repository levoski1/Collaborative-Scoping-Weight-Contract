// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IWaveScoping} from "@interfaces/IWaveScoping.sol";
import {IReputationManager} from "@interfaces/IReputationManager.sol";
import {ISlashingManager} from "@interfaces/ISlashingManager.sol";
import {ReputationManager} from "@modules/ReputationManager.sol";
import {EmergencyScoping} from "@modules/EmergencyScoping.sol";
import {SlashingManager} from "@modules/SlashingManager.sol";
import {MathLib} from "@libraries/MathLib.sol";
import {WaveLib} from "@libraries/WaveLib.sol";

contract WaveScoping is IWaveScoping {
    using MathLib for uint256;

    address public owner;
    ReputationManager public reputationManager;
    EmergencyScoping public emergencyScoping;
    SlashingManager public slashingManager;

    WaveLib.WaveConfig public waveConfig;

    mapping(string => Issue) public registry;
    mapping(address => mapping(string => uint256)) public votesBy;
    mapping(uint256 => WaveLib.WaveData) public waves;
    mapping(string => uint256) public issueToWave;

    uint256 public currentWaveId;
    uint256 public totalIssues;
    uint256 public totalVotes;

    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    modifier onlyOwnerOrEmergency() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    modifier waveIsActive(uint256 waveId) {
        WaveLib.WaveData storage wave = waves[waveId];
        if (!wave.isActive || wave.isFinalized) revert WaveNotActive();
        _;
    }

    constructor(
        uint256 _votingPeriodBlocks,
        uint256 _emergencyTimelock,
        uint256 _maxWeightPerVote,
        uint256 _pointsDivisor,
        uint256 _decayRateBps,
        uint256 _slashMaxBps,
        uint256 _slashBurnRateBps
    ) {
        owner = msg.sender;

        waveConfig = WaveLib.WaveConfig({
            votingPeriodBlocks: _votingPeriodBlocks,
            emergencyTimelock: _emergencyTimelock,
            maxWeightPerVote: _maxWeightPerVote,
            pointsDivisor: _pointsDivisor
        });

        reputationManager = new ReputationManager(address(this), _decayRateBps, _pointsDivisor);
        emergencyScoping = new EmergencyScoping(address(this), _emergencyTimelock);
        slashingManager = new SlashingManager(address(this), _slashMaxBps, _slashBurnRateBps);
    }

    function setOwner(address _newOwner) external onlyOwner {
        if (_newOwner == address(0)) revert ZeroAddress();
        owner = _newOwner;
    }

    function createWave(string calldata _name, uint256 _durationBlocks) external onlyOwner {
        uint256 waveId = WaveLib.encodeWaveId(_name, block.number);
        waves[waveId] = WaveLib.WaveData({
            id: waveId,
            name: _name,
            startBlock: block.number,
            endBlock: block.number + _durationBlocks,
            isActive: true,
            isFinalized: false,
            totalWeightCast: 0,
            issueCount: 0
        });
        currentWaveId = waveId;
        emit WaveCreated(waveId, _name, block.number + _durationBlocks);
    }

    function registerIssue(string calldata _url) external onlyOwner {
        if (registry[_url].exists) revert AlreadyRegistered();

        registry[_url] = Issue({
            githubIssueUrl: _url,
            currentWeight: 0,
            assignedPoints: 0,
            exists: true,
            isEmergency: false,
            assignedContributor: address(0),
            startedAtBlock: 0
        });

        issueToWave[_url] = currentWaveId;
        waves[currentWaveId].issueCount++;
        totalIssues++;

        emit IssueRegistered(_url, currentWaveId);
    }

    function voteOnIssue(string calldata _url, uint256 _weight) external {
        Issue storage issue = registry[_url];
        if (!issue.exists) revert NotRegistered();
        if (_weight == 0 || _weight > waveConfig.maxWeightPerVote) revert InvalidWeight();
        if (votesBy[msg.sender][_url] != 0) revert AlreadyVoted();

        WaveLib.WaveData storage wave = waves[issueToWave[_url]];
        if (!wave.isWithinVotingPeriod(waveConfig)) revert WaveNotActive();

        uint256 reputationWeight = reputationManager.reputationToWeight(
            reputationManager.balanceOf(msg.sender)
        );
        uint256 effectiveWeight = _weight.calculateWeightedScore(
            reputationWeight,
            waveConfig.pointsDivisor
        );
        effectiveWeight = MathLib.min(effectiveWeight, waveConfig.maxWeightPerVote);

        votesBy[msg.sender][_url] = effectiveWeight;
        issue.currentWeight += effectiveWeight;
        issue.assignedPoints = issue.currentWeight.calculatePoints(waveConfig.pointsDivisor);
        wave.totalWeightCast += effectiveWeight;

        reputationManager.mintReputation(msg.sender, _weight / 10 + 1);
        totalVotes++;

        emit Voted(msg.sender, _url, effectiveWeight);
    }

    function fastTrackIssue(string calldata _url, string calldata _reason) external onlyOwner {
        Issue storage issue = registry[_url];
        if (!issue.exists) revert NotRegistered();

        emergencyScoping.proposeFastTrack(_url, _reason, owner);
        emergencyScoping.executeFastTrack(_url, owner);

        issue.isEmergency = true;
        issue.currentWeight += waveConfig.maxWeightPerVote * 2;

        emit IssueFastTracked(_url, msg.sender);
    }

    function startWork(string calldata _url) external {
        Issue storage issue = registry[_url];
        if (!issue.exists) revert NotRegistered();
        if (issue.assignedContributor != address(0)) revert PointsAlreadyAssigned();

        issue.assignedContributor = msg.sender;
        issue.startedAtBlock = block.number;

        emit ContributionStarted(_url, msg.sender);
    }

    function finalizeWave(uint256 _waveId) external onlyOwner {
        WaveLib.WaveData storage wave = waves[_waveId];
        if (!wave.isActive) revert WaveNotActive();
        if (wave.isFinalized) revert WaveAlreadyFinalized();

        wave.isFinalized = true;

        emit WaveFinalized(_waveId);
    }

    function adjustPoints(
        string calldata _url,
        uint256 _newPoints,
        string calldata _reason
    ) external onlyOwner {
        Issue storage issue = registry[_url];
        if (!issue.exists) revert NotRegistered();
        if (issue.assignedContributor != address(0) && block.number <= issue.startedAtBlock + 5) {
            uint256 previousPoints = issue.assignedPoints;
            issue.assignedPoints = _newPoints;
            emit PointsAdjusted(_url, previousPoints, _newPoints);
        } else if (issue.assignedContributor == address(0)) {
            uint256 previousPoints = issue.assignedPoints;
            issue.assignedPoints = _newPoints;
            emit PointsAdjusted(_url, previousPoints, _newPoints);
        } else {
            uint256 previousPoints = issue.assignedPoints;
            slashingManager.slash(msg.sender, _url, previousPoints, _newPoints, _reason);
            issue.assignedPoints = _newPoints;
            emit PointsAdjusted(_url, previousPoints, _newPoints);
        }
    }

    function getIssue(string calldata _url)
        external
        view
        returns (uint256 weight, uint256 points, bool isEmergency, address contributor)
    {
        Issue storage i = registry[_url];
        if (!i.exists) revert NotRegistered();
        return (i.currentWeight, i.assignedPoints, i.isEmergency, i.assignedContributor);
    }

    function getWave(uint256 _waveId)
        external
        view
        returns (WaveLib.WaveData memory)
    {
        return waves[_waveId];
    }

    function getCurrentWave() external view returns (WaveLib.WaveData memory) {
        return waves[currentWaveId];
    }

    function getWaveConfig()
        external
        view
        returns (WaveLib.WaveConfig memory)
    {
        return waveConfig;
    }

    function getVoterWeight(address voter, string calldata _url) external view returns (uint256) {
        return votesBy[voter][_url];
    }
}
