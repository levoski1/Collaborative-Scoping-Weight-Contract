pragma solidity ^0.8.20;

import {forge-std/Test.sol} as Test;
import {ReputationManager} from "../src/modules/ReputationManager.sol";
import {IReputationManager} from "../src/interfaces/IReputationManager.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

contract ReputationManagerTest is Test, TestHelpers {
    ReputationManager public reputationManager;
    address public waveScoping;

    function setUp() public {
        waveScoping = address(this);
        reputationManager = new ReputationManager(waveScoping, DECAY_RATE, POINTS_DIVISOR);
    }

    function test_MintReputation() public {
        (address alice,,) = createTestUsers();
        reputationManager.mintReputation(alice, 100);
        assertEq(reputationManager.balanceOf(alice), 100);
    }

    function test_BurnReputation() public {
        (address alice,,) = createTestUsers();
        reputationManager.mintReputation(alice, 100);
        reputationManager.burnReputation(alice, 40);
        assertEq(reputationManager.balanceOf(alice), 60);
    }

    function test_Revert_BurnMoreThanBalance() public {
        (address alice,,) = createTestUsers();
        reputationManager.mintReputation(alice, 10);
        vm.expectRevert(
            abi.encodeWithSelector(
                IReputationManager.InsufficientReputation.selector,
                alice, 10, 100
            )
        );
        reputationManager.burnReputation(alice, 100);
    }

    function test_ConsumeReputation() public {
        (address alice,,) = createTestUsers();
        reputationManager.mintReputation(alice, 50);
        bool success = reputationManager.consumeReputation(alice, 30);
        assertEq(success, true);
        assertEq(reputationManager.balanceOf(alice), 20);
    }

    function test_ConsumeInsufficientReputation() public {
        (address alice,,) = createTestUsers();
        bool success = reputationManager.consumeReputation(alice, 10);
        assertEq(success, false);
    }

    function test_Decay() public {
        (address alice,,) = createTestUsers();
        reputationManager.mintReputation(alice, 100);

        reputationManager.decayAll();
        reputationManager.decayAll();

        reputationManager.mintReputation(alice, 0);

        uint256 balance = reputationManager.balanceOf(alice);
        assertLt(balance, 100);
        assertGt(balance, 95);
    }

    function test_Revert_NotWaveScoping() public {
        (address alice,,) = createTestUsers();
        vm.prank(alice);
        vm.expectRevert();
        reputationManager.mintReputation(alice, 100);
    }
}
