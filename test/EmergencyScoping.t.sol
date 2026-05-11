pragma solidity ^0.8.20;

import {forge-std/Test.sol} as Test;
import {EmergencyScoping} from "../src/modules/EmergencyScoping.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

contract EmergencyScopingTest is Test, TestHelpers {
    EmergencyScoping public emergencyScoping;
    address public waveScoping;
    address public owner;

    function setUp() public {
        waveScoping = address(this);
        owner = address(this);
        emergencyScoping = new EmergencyScoping(waveScoping, EMERGENCY_TIMELOCK);
    }

    function test_ProposeFastTrack() public {
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
        assertEq(emergencyScoping.proposalsCount(), 1);
    }

    function test_ExecuteFastTrack() public {
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
        mineBlocks(EMERGENCY_TIMELOCK + 1);
        emergencyScoping.executeFastTrack(ISSUE_URL_1, owner);

        (,,,, bool executed) = emergencyScoping.getProposal(ISSUE_URL_1);
        assertEq(executed, true);
    }

    function test_Revert_DoublePropose() public {
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
        vm.expectRevert();
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
    }

    function test_Revert_TimelockNotMet() public {
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
        mineBlocks(EMERGENCY_TIMELOCK - 1);
        vm.expectRevert();
        emergencyScoping.executeFastTrack(ISSUE_URL_1, owner);
    }

    function test_Revert_NotOwner() public {
        (address alice,,) = createTestUsers();
        vm.prank(alice);
        vm.expectRevert();
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
    }

    function test_GetProposalCount() public {
        assertEq(emergencyScoping.getProposalCount(), 0);
        emergencyScoping.proposeFastTrack(ISSUE_URL_1, EMERGENCY_REASON, owner);
        assertEq(emergencyScoping.getProposalCount(), 1);
        emergencyScoping.proposeFastTrack(ISSUE_URL_2, "Another reason", owner);
        assertEq(emergencyScoping.getProposalCount(), 2);
    }
}
