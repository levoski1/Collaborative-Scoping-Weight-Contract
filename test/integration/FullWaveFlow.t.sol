pragma solidity ^0.8.20;

import {forge-std/Test.sol} as Test;
import {WaveScoping} from "../../src/WaveScoping.sol";
import {IWaveScoping} from "../../src/interfaces/IWaveScoping.sol";
import {IReputationManager} from "../../src/interfaces/IReputationManager.sol";
import {TestHelpers} from "../helpers/TestHelpers.sol";

contract FullWaveFlowTest is Test, TestHelpers {
    WaveScoping public waveScoping;
    address public owner;
    address public alice;
    address public bob;
    address public charlie;

    function setUp() public {
        owner = address(this);
        (alice, bob, charlie) = createTestUsers();

        waveScoping = new WaveScoping(
            VOTING_PERIOD,
            EMERGENCY_TIMELOCK,
            MAX_WEIGHT,
            POINTS_DIVISOR,
            DECAY_RATE,
            SLASH_MAX_BPS,
            SLASH_BURN_RATE
        );
    }

    function test_FullWaveLifecycle() public {
        waveScoping.createWave("Sprint 2026.2", VOTING_PERIOD);
        uint256 waveId = 102190192228077601822963497419233938830335651498641448493105269110191523618002;

        waveScoping.registerIssue(ISSUE_URL_1);
        waveScoping.registerIssue(ISSUE_URL_2);

        vm.prank(alice);
        waveScoping.voteOnIssue(ISSUE_URL_1, 30);

        vm.prank(bob);
        waveScoping.voteOnIssue(ISSUE_URL_1, 20);

        vm.prank(charlie);
        waveScoping.voteOnIssue(ISSUE_URL_2, 50);

        (uint256 w1, uint256 p1,,) = waveScoping.getIssue(ISSUE_URL_1);
        assertEq(w1, 50);
        assertEq(p1, 5);

        (uint256 w2, uint256 p2,,) = waveScoping.getIssue(ISSUE_URL_2);
        assertEq(w2, 50);
        assertEq(p2, 5);

        vm.prank(alice);
        waveScoping.startWork(ISSUE_URL_1);

        waveScoping.finalizeWave(waveId);

        IReputationManager repManager =
            IReputationManager(address(waveScoping.reputationManager()));
        assertGt(repManager.balanceOf(alice), 0);
        assertGt(repManager.balanceOf(bob), 0);
        assertGt(repManager.balanceOf(charlie), 0);
    }

    function test_FastTrackDuringWave() public {
        waveScoping.createWave("Sprint Hotfix", VOTING_PERIOD);
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        waveScoping.voteOnIssue(ISSUE_URL_1, 10);

        uint256 weightBefore;
        (weightBefore,,,,) = waveScoping.getIssue(ISSUE_URL_1);

        waveScoping.fastTrackIssue(ISSUE_URL_1, EMERGENCY_REASON);

        (uint256 weightAfter,, bool isEmergency,) = waveScoping.getIssue(ISSUE_URL_1);
        assertTrue(isEmergency);
        assertGt(weightAfter, weightBefore);
    }

    function test_SlashOnLatePointChange() public {
        waveScoping.createWave("Sprint Safe", VOTING_PERIOD);
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        waveScoping.startWork(ISSUE_URL_1);

        mineBlocks(10);

        vm.expectRevert();
        waveScoping.adjustPoints(ISSUE_URL_1, 100, "Late increase");
    }

    function test_ReputationWeightedVoting() public {
        waveScoping.createWave("Sprint Rep", VOTING_PERIOD);
        waveScoping.registerIssue(ISSUE_URL_1);

        IReputationManager repManager =
            IReputationManager(address(waveScoping.reputationManager()));

        repManager.mintReputation(alice, 100);

        vm.prank(alice);
        waveScoping.voteOnIssue(ISSUE_URL_1, 30);

        (uint256 weight,,,) = waveScoping.getIssue(ISSUE_URL_1);
        assertGe(weight, 30);

        uint256 aliceVote = waveScoping.getVoterWeight(alice, ISSUE_URL_1);
        assertGe(aliceVote, 30);
        assertLe(aliceVote, MAX_WEIGHT);
    }

    function test_MultipleWaves() public {
        waveScoping.createWave("Wave 1", VOTING_PERIOD);
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        waveScoping.voteOnIssue(ISSUE_URL_1, 50);

        (uint256 w1,,,) = waveScoping.getIssue(ISSUE_URL_1);
        assertEq(w1, 50);

        waveScoping.createWave("Wave 2", VOTING_PERIOD);
        waveScoping.registerIssue(ISSUE_URL_2);

        vm.prank(bob);
        waveScoping.voteOnIssue(ISSUE_URL_2, 75);

        waveScoping.finalizeWave(waveScoping.currentWaveId());
    }
}
