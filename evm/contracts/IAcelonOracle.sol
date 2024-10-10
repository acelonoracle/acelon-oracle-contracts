// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./AcelonStructs.sol";

interface IAcelonOracle {
    function updatePriceFeeds(bytes[] calldata updateData, bytes[][] calldata signature) external;

    function priceFeedExists(bytes32 requestHash) external view returns (bool);
    
    function getValidTimePeriod() external view returns (uint64 validTimePeriod);

    function getPrice(bytes32 requestHash) external view returns (AcelonStructs.PriceEntry memory priceEntry);

    function getPriceNoOlderThan(bytes32 requestHash, uint64 age) external view returns (AcelonStructs.PriceEntry memory priceEntry);

    event PriceFeedUpdate(
        bytes32 indexed requestHash,
        AcelonStructs.PriceEntry priceEntry
    );
}