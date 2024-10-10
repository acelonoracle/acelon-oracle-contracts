pragma solidity ^0.8.0;

contract AcelonStructs {
    struct PriceEntry {
        uint64 timestamp;
        uint128[] prices;
    }

    struct PricePayload {
        uint128[] prices;
        uint64 timestamp;
        bytes32[] certificates;
        bytes32 requestHash;
    }
}

