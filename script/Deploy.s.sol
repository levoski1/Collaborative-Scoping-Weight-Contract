pragma solidity ^0.8.20;

import {forge-std/Script.sol} as Script;
import {WaveScoping} from "../src/WaveScoping.sol";

contract DeployScript is Script {
    uint256 constant VOTING_PERIOD_BLOCKS = 64800;
    uint256 constant EMERGENCY_TIMELOCK = 5;
    uint256 constant MAX_WEIGHT_PER_VOTE = 100;
    uint256 constant POINTS_DIVISOR = 10;
    uint256 constant DECAY_RATE_BPS = 100;
    uint256 constant SLASH_MAX_BPS = 2000;
    uint256 constant SLASH_BURN_RATE_BPS = 5000;

    function run() external returns (WaveScoping) {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        WaveScoping waveScoping = new WaveScoping(
            VOTING_PERIOD_BLOCKS,
            EMERGENCY_TIMELOCK,
            MAX_WEIGHT_PER_VOTE,
            POINTS_DIVISOR,
            DECAY_RATE_BPS,
            SLASH_MAX_BPS,
            SLASH_BURN_RATE_BPS
        );

        vm.stopBroadcast();

        return waveScoping;
    }
}

contract DeployAndCreateWave is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        string memory waveName = vm.envString("WAVE_NAME");
        uint256 durationBlocks = vm.envUint("WAVE_DURATION_BLOCKS");

        vm.startBroadcast(deployerPrivateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.createWave(waveName, durationBlocks);

        vm.stopBroadcast();
    }
}
