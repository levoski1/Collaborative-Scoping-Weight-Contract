pragma solidity ^0.8.20;

import {ISlashingManager} from "@interfaces/ISlashingManager.sol";
import {MathLib} from "@libraries/MathLib.sol";

contract SlashingManager is ISlashingManager {
    using MathLib for uint256;

    address public immutable waveScoping;
    uint256 public immutable maxPenaltyBps;
    uint256 public immutable burnRateBps;

    mapping(address => mapping(string => SlashRecord)) private _slashRecords;
    mapping(address => uint256) private _totalSlashed;

    modifier onlyWaveScoping() {
        if (msg.sender != waveScoping) revert NotAuthorized();
        _;
    }

    event MaxPenaltyUpdated(uint256 oldMax, uint256 newMax);

    constructor(address _waveScoping, uint256 _maxPenaltyBps, uint256 _burnRateBps) {
        if (_waveScoping == address(0)) revert ZeroAddress();
        if (_maxPenaltyBps > MathLib.BASIS_POINTS) revert PenaltyExceedsMax();
        waveScoping = _waveScoping;
        maxPenaltyBps = _maxPenaltyBps;
        burnRateBps = _burnRateBps;
    }

    function slash(
        address maintainer,
        string calldata issueUrl,
        uint256 previousPoints,
        uint256 newPoints,
        string calldata reason
    ) external onlyWaveScoping returns (uint256 penalty) {
        if (newPoints <= previousPoints) revert NoChangeDetected();
        if (_slashRecords[maintainer][issueUrl].applied) revert DuplicateSlash();

        uint256 pointDiff = newPoints - previousPoints;
        penalty = pointDiff.applyBasisPoints(maxPenaltyBps);
        if (penalty == 0) penalty = 1;

        uint256 burnAmount = penalty.applyBasisPoints(burnRateBps);
        uint256 slashedAmount = penalty - burnAmount;

        _slashRecords[maintainer][issueUrl] = SlashRecord({
            maintainer: maintainer,
            issueUrl: issueUrl,
            penalty: penalty,
            timestamp: block.timestamp,
            applied: true
        });
        _totalSlashed[maintainer] += penalty;

        emit MaintainerSlashed(maintainer, issueUrl, penalty, reason);
        return penalty;
    }

    function getSlashRecord(
        address maintainer,
        string calldata issueUrl
    ) external view returns (SlashRecord memory) {
        return _slashRecords[maintainer][issueUrl];
    }

    function totalSlashed(address maintainer) external view returns (uint256) {
        return _totalSlashed[maintainer];
    }
}
