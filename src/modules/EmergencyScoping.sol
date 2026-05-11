pragma solidity ^0.8.20;

contract EmergencyScoping {
    address public immutable waveScoping;
    uint256 public immutable timelockBlocks;

    struct FastTrackProposal {
        string issueUrl;
        address proposer;
        uint256 proposedAt;
        bool executed;
        string reason;
    }

    mapping(string => FastTrackProposal) public proposals;
    string[] private _proposalList;
    uint256 public proposalsCount;

    event FastTrackProposed(string indexed url, address indexed proposer, string reason);
    event FastTrackExecuted(string indexed url, uint256 timestamp);

    error NotAuthorized();
    error AlreadyProposed();
    error TimelockNotMet();
    error AlreadyExecuted();
    error NotProposer();
    error EmptyReason();

    modifier onlyWaveScoping() {
        if (msg.sender != waveScoping) revert NotAuthorized();
        _;
    }

    modifier onlyOwner(address owner) {
        if (msg.sender != owner) revert NotAuthorized();
        _;
    }

    constructor(address _waveScoping, uint256 _timelockBlocks) {
        if (_waveScoping == address(0)) revert ZeroAddress();
        waveScoping = _waveScoping;
        timelockBlocks = _timelockBlocks;
    }

    function proposeFastTrack(
        string calldata _url,
        string calldata _reason,
        address _owner
    ) external onlyOwner(_owner) {
        if (proposals[_url].proposedAt != 0) revert AlreadyProposed();
        if (bytes(_reason).length == 0) revert EmptyReason();

        proposals[_url] = FastTrackProposal({
            issueUrl: _url,
            proposer: msg.sender,
            proposedAt: block.number,
            executed: false,
            reason: _reason
        });
        _proposalList.push(_url);
        proposalsCount++;

        emit FastTrackProposed(_url, msg.sender, _reason);
    }

    function executeFastTrack(string calldata _url, address _owner) external onlyOwner(_owner) {
        FastTrackProposal storage prop = proposals[_url];
        if (prop.proposedAt == 0) revert NotAuthorized();
        if (prop.executed) revert AlreadyExecuted();
        if (block.number < prop.proposedAt + timelockBlocks) revert TimelockNotMet();

        prop.executed = true;
        emit FastTrackExecuted(_url, block.timestamp);
    }

    function getProposal(string calldata _url) external view returns (FastTrackProposal memory) {
        return proposals[_url];
    }

    function getProposalCount() external view returns (uint256) {
        return _proposalList.length;
    }

    function isFastTracked(string calldata _url) external view returns (bool) {
        FastTrackProposal storage prop = proposals[_url];
        return prop.executed;
    }
}
