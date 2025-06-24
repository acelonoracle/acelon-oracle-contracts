// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./AcelonStructs.sol";
import "./IAcelonOracle.sol";

interface AggregatorV3Interface {
    function decimals() external view returns (uint8);
    function description() external view returns (string memory);
    function version() external view returns (uint256);

    function latestRoundData() external view returns (
        uint80 roundId,
        int256 answer,
        uint256 startedAt,
        uint256 updatedAt,
        uint80 answeredInRound
    );
}


contract AcelonAggregator is AggregatorV3Interface {
    struct Source {
        IAcelonOracle oracle;
        bytes32 requestHash;
        uint8 feedIndex;
        bool inverted;
    }

    Source[] public sources;
    string private _description;
    uint8 private _decimals;
    uint256 private _version = 1;

    constructor(
        Source[] memory sources_,
        string memory description_,
        uint8 decimals_
    ) {
        require(sources_.length > 0, "At least one source required");
        for (uint i = 0; i < sources_.length; i++) {
            sources.push(sources_[i]);
        }
        _description = description_;
        _decimals = decimals_;
    }

    function decimals() external view override returns (uint8) {
        return _decimals;
    }

    function description() external view override returns (string memory) {
        return _description;
    }

    function version() external view override returns (uint256) {
        return _version;
    }

    function latestRoundData()
        public
        view
        override
        returns (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        )
    {
        uint256 result = 1e18;
        uint64 latestTimestamp = 0;

        for (uint i = 0; i < sources.length; i++) {
            Source memory source = sources[i];
            AcelonStructs.PriceEntry memory entry = source.oracle.getPrice(source.requestHash);
            require(entry.prices.length > source.feedIndex, "Invalid feed index");

            uint256 price = uint256(entry.prices[source.feedIndex]);
            require(price > 0, "Invalid price");

            if (source.inverted) {
                result = (result * 1e18) / price;
            } else {
                result = (result * price) / 1e18;
            }

            if (entry.timestamp > latestTimestamp) {
                latestTimestamp = entry.timestamp;
            }
        }

        answer = int256(result);
        updatedAt = uint256(latestTimestamp);
        startedAt = updatedAt;
        roundId = uint80(latestTimestamp); // Optional: hash could be used
        answeredInRound = roundId;
    }

}
