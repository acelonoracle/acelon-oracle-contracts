// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

contract AcelonOwner {
    mapping(address => bool) public owners;
    mapping(address => bool) public proposedOwners;

    constructor(address[] memory _owners) {
        require(_owners.length > 0, "At least one owner required");
        for(uint8 i = 0; i < _owners.length; i++) {
            owners[_owners[i]] = true;
        }
    }

    modifier onlyOwner() {
        require(owners[msg.sender], "NOT_OWNER");
        _;
    }

    modifier onlyProposedOwner() {
        require(proposedOwners[msg.sender], "NOT_PROPOSED_OWNER");
        _;
    }

    function proposeOwner(address newOwner) public onlyOwner {
        proposedOwners[newOwner] = true;
    }

    function acceptOwner() public onlyProposedOwner {
        proposedOwners[msg.sender] = false;
        owners[msg.sender] = true;
    }

    function removeOwner(address owner) public onlyOwner {
        owners[owner] = false;
    }
}