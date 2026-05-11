pragma solidity ^0.8.20;

import {forge-std/Test.sol} as Test;

contract TestHelpers is Test {
    string constant ISSUE_URL_1 = "https://github.com/owner/repo/issues/1";
    string constant ISSUE_URL_2 = "https://github.com/owner/repo/issues/2";
    string constant ISSUE_URL_3 = "https://github.com/owner/repo/issues/3";
    string constant EMERGENCY_REASON = "Critical security vulnerability in auth module";
    string constant WAVE_NAME = "Sprint 2026.1";

    uint256 constant VOTING_PERIOD = 1000;
    uint256 constant EMERGENCY_TIMELOCK = 5;
    uint256 constant MAX_WEIGHT = 100;
    uint256 constant POINTS_DIVISOR = 10;
    uint256 constant DECAY_RATE = 100;
    uint256 constant SLASH_MAX_BPS = 2000;
    uint256 constant SLASH_BURN_RATE = 5000;

    function createTestUsers() internal returns (address alice, address bob, address charlie) {
        alice = makeAddr("alice");
        bob = makeAddr("bob");
        charlie = makeAddr("charlie");
        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);
        vm.deal(charlie, 10 ether);
    }

    function mineBlocks(uint256 n) internal {
        for (uint256 i = 0; i < n; i++) {
            vm.roll(block.number + 1);
        }
    }
}
