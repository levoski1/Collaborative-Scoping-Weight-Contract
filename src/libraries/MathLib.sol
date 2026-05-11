pragma solidity ^0.8.20;

library MathLib {
    uint256 public constant BASIS_POINTS = 10_000;

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    function max(uint256 a, uint256 b) internal pure returns (uint256) {
        return a > b ? a : b;
    }

    function applyBasisPoints(uint256 value, uint256 bp) internal pure returns (uint256) {
        return (value * bp) / BASIS_POINTS;
    }

    function calculateWeightedScore(
        uint256 baseWeight,
        uint256 reputationBonus,
        uint256 reputationDivisor
    ) internal pure returns (uint256) {
        return baseWeight + (reputationBonus / reputationDivisor);
    }

    function calculatePoints(uint256 weight, uint256 divisor) internal pure returns (uint256) {
        return weight / divisor;
    }

    function decay(uint256 value, uint256 decayBasisPoints) internal pure returns (uint256) {
        uint256 retained = BASIS_POINTS - decayBasisPoints;
        return (value * retained) / BASIS_POINTS;
    }

    function isInRange(uint256 value, uint256 minV, uint256 maxV) internal pure returns (bool) {
        return value >= minV && value <= maxV;
    }
}
