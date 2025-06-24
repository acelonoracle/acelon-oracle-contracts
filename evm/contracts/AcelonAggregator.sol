// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./AcelonStructs.sol";
import "./IAcelonOracle.sol";
import "./Owner.sol";

interface AggregatorV3Interface {
    function decimals() external view returns (uint8);
    function description() external view returns (string memory);
    function version() external view returns (uint256);

    function getRoundData(
        uint80 _roundId
    ) external view returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound);


    function latestRoundData() external view returns (
        uint80 roundId,
        int256 answer,
        uint256 startedAt,
        uint256 updatedAt,
        uint80 answeredInRound
    );
}

contract AcelonAggregator is AggregatorV3Interface, AcelonOwner {
    struct Source {
        IAcelonOracle oracle;
        bytes32 requestHash;
        uint8 feedIndex;
        uint256 precisionFactor;
        bool inverted;
    }

    Source[] public sources;
    string private _description;
    uint8 private _decimals;
    uint256 private _version = 1;

    constructor(address[] memory _owners, Source[] memory _sources, string memory description_, uint8 decimals_) AcelonOwner(_owners) {
        require(_sources.length > 0, "At least one source required");
        for (uint i = 0; i < _sources.length; i++) {
            sources.push(_sources[i]);
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

    function getRoundData(uint80) external view override returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound) {
        return latestRoundData();
    }

    function latestRoundData() public view override returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound) {
        uint256 precisionFactor = uint256(10) ** uint256(_decimals);
        uint256 price = precisionFactor;
        uint64 latestTimestamp = 0;

        for (uint i = 0; i < sources.length; i++) {
            Source memory source = sources[i];
            AcelonStructs.PriceEntry memory entry = source.oracle.getPrice(source.requestHash);
            require(entry.prices.length > source.feedIndex, "Invalid feed index");

            uint256 fetchedPrice = uint256(entry.prices[source.feedIndex]);
            require(price > 0, "Invalid price");

            uint256 localPrice = fetchedPrice * precisionFactor / source.precisionFactor;

            if (source.inverted) {
                price = (price * precisionFactor * precisionFactor) / localPrice;
            } else {
                price = (price * localPrice) / precisionFactor;
            }

            if (entry.timestamp > latestTimestamp) {
                latestTimestamp = entry.timestamp;
            }
        }

        roundId = uint80(latestTimestamp);
        answer = int256(price);
        updatedAt = uint256(latestTimestamp);
        startedAt = updatedAt;
        answeredInRound = roundId;
    }

    function sourceCount() external view returns (uint256) {
        return sources.length;
    }

    function getSource(uint256 index) external view returns (Source memory) {
        require(index < sources.length, "Out of bounds");
        return sources[index];
    }

    function updateSources(Source[] memory _sources) public onlyOwner {
        delete sources;
        for (uint i = 0; i < _sources.length; i++) {
            sources.push(_sources[i]);
        }
    }
}
