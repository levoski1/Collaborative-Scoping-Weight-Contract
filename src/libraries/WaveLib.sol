pragma solidity ^0.8.20;

library WaveLib {
    struct WaveData {
        uint256 id;
        string name;
        uint256 startBlock;
        uint256 endBlock;
        bool isActive;
        bool isFinalized;
        uint256 totalWeightCast;
        uint256 issueCount;
    }

    struct WaveConfig {
        uint256 votingPeriodBlocks;
        uint256 emergencyTimelock;
        uint256 maxWeightPerVote;
        uint256 pointsDivisor;
    }

    function isWithinVotingPeriod(WaveData storage wave, WaveConfig storage config) internal view returns (bool) {
        return
            wave.isActive &&
            !wave.isFinalized &&
            block.number >= wave.startBlock &&
            block.number <= wave.endBlock;
    }

    function hasVotingEnded(WaveData storage wave) internal view returns (bool) {
        return block.number > wave.endBlock;
    }

    function validateWaveConfig(WaveConfig storage config) internal pure returns (bool) {
        return
            config.votingPeriodBlocks > 0 &&
            config.emergencyTimelock > 0 &&
            config.maxWeightPerVote > 0 &&
            config.pointsDivisor > 0;
    }

    function encodeWaveId(string memory name, uint256 startBlock) internal pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(name, startBlock)));
    }
}
