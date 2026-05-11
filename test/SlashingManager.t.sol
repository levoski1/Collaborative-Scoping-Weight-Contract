pragma solidity ^0.8.20;

import {forge-std/Test.sol} as Test;
import {SlashingManager} from "../src/modules/SlashingManager.sol";
import {ISlashingManager} from "../src/interfaces/ISlashingManager.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

contract SlashingManagerTest is Test, TestHelpers {
    SlashingManager public slashingManager;
    address public waveScoping;
    address public maintainer;

    function setUp() public {
        waveScoping = address(this);
        maintainer = makeAddr("maintainer");
        slashingManager = new SlashingManager(waveScoping, SLASH_MAX_BPS, SLASH_BURN_RATE);
    }

    function test_Slash() public {
        uint256 penalty = slashingManager.slash(
            maintainer,
            ISSUE_URL_1,
            10,
            50,
            "Unauthorized point change"
        );

        assertGt(penalty, 0);
        assertEq(slashingManager.totalSlashed(maintainer), penalty);
    }

    function test_Revert_DuplicateSlash() public {
        slashingManager.slash(maintainer, ISSUE_URL_1, 10, 50, "First slash");
        vm.expectRevert(ISlashingManager.DuplicateSlash.selector);
        slashingManager.slash(maintainer, ISSUE_URL_1, 10, 50, "Second slash");
    }

    function test_Revert_NoChangeDetected() public {
        vm.expectRevert(ISlashingManager.NoChangeDetected.selector);
        slashingManager.slash(maintainer, ISSUE_URL_1, 50, 30, "Decrease not allowed");
    }

    function test_SlashRecords() public {
        slashingManager.slash(maintainer, ISSUE_URL_1, 10, 50, "Reason");

        ISlashingManager.SlashRecord memory record =
            slashingManager.getSlashRecord(maintainer, ISSUE_URL_1);
        assertEq(record.maintainer, maintainer);
        assertEq(record.applied, true);
        assertEq(record.penalty, (50 - 10) * SLASH_MAX_BPS / 10000);
    }

    function test_TotalSlashedAggregation() public {
        slashingManager.slash(maintainer, ISSUE_URL_1, 10, 50, "First");
        uint256 firstPenalty = slashingManager.totalSlashed(maintainer);

        slashingManager.slash(maintainer, ISSUE_URL_2, 5, 25, "Second");
        uint256 secondPenalty = slashingManager.totalSlashed(maintainer);

        assertGt(secondPenalty, firstPenalty);
    }

    function test_Revert_NotAuthorized() public {
        (address alice,,) = createTestUsers();
        vm.prank(alice);
        vm.expectRevert(ISlashingManager.NotAuthorized.selector);
        slashingManager.slash(maintainer, ISSUE_URL_1, 10, 50, "Unauthorized");
    }
}
