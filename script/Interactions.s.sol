pragma solidity ^0.8.20;

import {forge-std/Script.sol} as Script;
import {WaveScoping} from "../src/WaveScoping.sol";

contract RegisterIssue is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        string memory issueUrl = vm.envString("ISSUE_URL");

        vm.startBroadcast(privateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.registerIssue(issueUrl);

        vm.stopBroadcast();
    }
}

contract VoteOnIssue is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        string memory issueUrl = vm.envString("ISSUE_URL");
        uint256 weight = vm.envUint("VOTE_WEIGHT");

        vm.startBroadcast(privateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.voteOnIssue(issueUrl, weight);

        vm.stopBroadcast();
    }
}

contract FastTrackIssue is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        string memory issueUrl = vm.envString("ISSUE_URL");
        string memory reason = vm.envString("REASON");

        vm.startBroadcast(privateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.fastTrackIssue(issueUrl, reason);

        vm.stopBroadcast();
    }
}

contract FinalizeWave is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        uint256 waveId = vm.envUint("WAVE_ID");

        vm.startBroadcast(privateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.finalizeWave(waveId);

        vm.stopBroadcast();
    }
}

contract StartWork is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        string memory issueUrl = vm.envString("ISSUE_URL");

        vm.startBroadcast(privateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.startWork(issueUrl);

        vm.stopBroadcast();
    }
}

contract AdjustPoints is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address waveScopingAddress = vm.envAddress("WAVE_SCOPING_ADDRESS");
        string memory issueUrl = vm.envString("ISSUE_URL");
        uint256 newPoints = vm.envUint("NEW_POINTS");
        string memory reason = vm.envString("REASON");

        vm.startBroadcast(privateKey);

        WaveScoping waveScoping = WaveScoping(waveScopingAddress);
        waveScoping.adjustPoints(issueUrl, newPoints, reason);

        vm.stopBroadcast();
    }
}
