pragma solidity ^0.8.20;

import {forge-std/Test.sol} as Test;
import {WaveScoping} from "../src/WaveScoping.sol";
import {IWaveScoping} from "../src/interfaces/IWaveScoping.sol";
import {MathLib} from "../src/libraries/MathLib.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

contract WaveScopingTest is Test, TestHelpers {
    WaveScoping public waveScoping;
    address public owner;

    event IssueRegistered(string indexed url, uint256 waveId);
    event Voted(address indexed voter, string indexed url, uint256 weight);
    event WaveCreated(uint256 indexed id, string name, uint256 endBlock);
    event IssueFastTracked(string indexed url, address indexed maintainer);

    function setUp() public {
        owner = address(this);

        waveScoping = new WaveScoping(
            VOTING_PERIOD,
            EMERGENCY_TIMELOCK,
            MAX_WEIGHT,
            POINTS_DIVISOR,
            DECAY_RATE,
            SLASH_MAX_BPS,
            SLASH_BURN_RATE
        );

        waveScoping.createWave(WAVE_NAME, VOTING_PERIOD);
    }

    function test_Constructor() public {
        assertEq(waveScoping.owner(), owner);
        assertEq(address(waveScoping.reputationManager()).code.length > 0, true);
        assertEq(address(waveScoping.emergencyScoping()).code.length > 0, true);
        assertEq(address(waveScoping.slashingManager()).code.length > 0, true);
    }

    function test_RegisterIssue() public {
        vm.expectEmit(true, false, false, true);
        emit IssueRegistered(ISSUE_URL_1, 0);
        waveScoping.registerIssue(ISSUE_URL_1);

        (uint256 weight, uint256 points, bool isEmergency, address contributor) =
            waveScoping.getIssue(ISSUE_URL_1);
        assertEq(weight, 0);
        assertEq(points, 0);
        assertEq(isEmergency, false);
        assertEq(contributor, address(0));
    }

    function test_Revert_DoubleRegister() public {
        waveScoping.registerIssue(ISSUE_URL_1);
        vm.expectRevert(IWaveScoping.AlreadyRegistered.selector);
        waveScoping.registerIssue(ISSUE_URL_1);
    }

    function test_Vote() public {
        (address alice,,) = createTestUsers();
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        vm.expectEmit(true, true, false, true);
        emit Voted(alice, ISSUE_URL_1, 10);
        waveScoping.voteOnIssue(ISSUE_URL_1, 10);

        (uint256 weight, uint256 points,,) = waveScoping.getIssue(ISSUE_URL_1);
        assertEq(weight, 10);
        assertEq(points, 1);
    }

    function test_Revert_DoubleVote() public {
        (address alice,,) = createTestUsers();
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        waveScoping.voteOnIssue(ISSUE_URL_1, 10);

        vm.prank(alice);
        vm.expectRevert(IWaveScoping.AlreadyVoted.selector);
        waveScoping.voteOnIssue(ISSUE_URL_1, 5);
    }

    function test_Revert_VoteUnregisteredIssue() public {
        (address alice,,) = createTestUsers();
        vm.prank(alice);
        vm.expectRevert(IWaveScoping.NotRegistered.selector);
        waveScoping.voteOnIssue(ISSUE_URL_1, 10);
    }

    function test_Revert_InvalidWeight() public {
        waveScoping.registerIssue(ISSUE_URL_1);
        vm.expectRevert(IWaveScoping.InvalidWeight.selector);
        waveScoping.voteOnIssue(ISSUE_URL_1, 0);

        vm.expectRevert(IWaveScoping.InvalidWeight.selector);
        waveScoping.voteOnIssue(ISSUE_URL_1, 101);
    }

    function test_OwnerTransfer() public {
        (address alice,,) = createTestUsers();
        waveScoping.setOwner(alice);
        assertEq(waveScoping.owner(), alice);

        vm.prank(alice);
        waveScoping.setOwner(owner);
    }

    function test_Revert_NotOwner() public {
        (address alice,,) = createTestUsers();
        vm.prank(alice);
        vm.expectRevert(IWaveScoping.NotOwner.selector);
        waveScoping.registerIssue(ISSUE_URL_1);
    }

    function test_StartWork() public {
        (address alice,,) = createTestUsers();
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        waveScoping.startWork(ISSUE_URL_1);

        (,,, address contributor) = waveScoping.getIssue(ISSUE_URL_1);
        assertEq(contributor, alice);
    }

    function test_Revert_StartWorkTwice() public {
        (address alice, address bob,) = createTestUsers();
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.prank(alice);
        waveScoping.startWork(ISSUE_URL_1);

        vm.prank(bob);
        vm.expectRevert(IWaveScoping.PointsAlreadyAssigned.selector);
        waveScoping.startWork(ISSUE_URL_1);
    }

    function test_VoteOutOfPeriod() public {
        (address alice,,) = createTestUsers();
        waveScoping.registerIssue(ISSUE_URL_1);

        mineBlocks(VOTING_PERIOD + 1);

        vm.prank(alice);
        vm.expectRevert(IWaveScoping.WaveNotActive.selector);
        waveScoping.voteOnIssue(ISSUE_URL_1, 10);
    }

    function test_FinalizeWave() public {
        waveScoping.finalizeWave(1);
        (, , , , bool isActive, bool isFinalized,,,,) =
            waveScoping.getWave(1);
        assertEq(isFinalized, true);
    }

    function test_FastTrackIssue() public {
        waveScoping.registerIssue(ISSUE_URL_1);

        vm.expectEmit(true, true, false, true);
        emit IssueFastTracked(ISSUE_URL_1, owner);
        waveScoping.fastTrackIssue(ISSUE_URL_1, EMERGENCY_REASON);

        (uint256 weight,, bool isEmergency,) = waveScoping.getIssue(ISSUE_URL_1);
        assertEq(isEmergency, true);
        assertGt(weight, 0);
    }

    function test_AdjustPointsWithoutContributor() public {
        waveScoping.registerIssue(ISSUE_URL_1);
        waveScoping.adjustPoints(ISSUE_URL_1, 50, "Initial adjustment");

        (uint256 weight, uint256 points,,) = waveScoping.getIssue(ISSUE_URL_1);
        assertEq(weight, 0);
        assertEq(points, 50);
    }

    function test_UserCannotRegisterIssue() public {
        (address alice,,) = createTestUsers();
        vm.prank(alice);
        vm.expectRevert(IWaveScoping.NotOwner.selector);
        waveScoping.registerIssue(ISSUE_URL_1);
    }
}
