pragma solidity ^0.8.20;

import {IReputationManager} from "@interfaces/IReputationManager.sol";
import {MathLib} from "@libraries/MathLib.sol";

contract ReputationManager is IReputationManager {
    using MathLib for uint256;

    address public immutable waveScoping;
    uint256 public immutable decayRateBps;
    uint256 public immutable reputationWeightDivisor;

    mapping(address => uint256) private _balances;
    mapping(address => uint256) private _lastDecayEpoch;
    uint256 public currentEpoch;

    modifier onlyWaveScoping() {
        if (msg.sender != waveScoping) revert NotAuthorized();
        _;
    }

    constructor(address _waveScoping, uint256 _decayRateBps, uint256 _divisor) {
        if (_waveScoping == address(0)) revert ZeroAddress();
        waveScoping = _waveScoping;
        decayRateBps = _decayRateBps;
        reputationWeightDivisor = _divisor;
    }

    function balanceOf(address user) external view returns (uint256) {
        return _balances[user];
    }

    function mintReputation(address user, uint256 amount) external onlyWaveScoping {
        if (user == address(0)) revert ZeroAddress();
        _applyDecay(user);
        _balances[user] += amount;
        emit ReputationMinted(user, amount);
    }

    function burnReputation(address user, uint256 amount) external onlyWaveScoping {
        if (user == address(0)) revert ZeroAddress();
        _applyDecay(user);
        if (_balances[user] < amount) {
            revert InsufficientReputation(user, _balances[user], amount);
        }
        _balances[user] -= amount;
        emit ReputationBurned(user, amount);
    }

    function consumeReputation(address user, uint256 amount) external onlyWaveScoping returns (bool) {
        if (user == address(0)) revert ZeroAddress();
        _applyDecay(user);
        if (_balances[user] < amount) {
            return false;
        }
        _balances[user] -= amount;
        emit ReputationBurned(user, amount);
        return true;
    }

    function decayAll() external {
        currentEpoch++;
        emit EpochAdvanced(currentEpoch);
    }

    function reputationToWeight(uint256 reputation) external pure returns (uint256) {
        return reputation;
    }

    function _applyDecay(address user) internal {
        uint256 epochsPassed = currentEpoch - _lastDecayEpoch[user];
        if (epochsPassed == 0) return;
        if (epochsPassed > 100) epochsPassed = 100;

        uint256 balance = _balances[user];
        for (uint256 i = 0; i < epochsPassed; i++) {
            balance = balance.decay(decayRateBps);
        }
        _balances[user] = balance;
        _lastDecayEpoch[user] = currentEpoch;
        emit ReputationDecayed(user, balance);
    }
}
