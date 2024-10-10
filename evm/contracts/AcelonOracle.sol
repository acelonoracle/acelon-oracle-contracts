// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./AcelonStructs.sol";
import "./IAcelonOracle.sol";

contract AcelonOracle is IAcelonOracle {
    bytes public constant ACURAST_SIGNATURE_PREFIX = "acusig";
    bytes public constant ACURAST_SCRIPT_PREFIX = "ipfs://QmVHRimsTBSEASEcnbd5MYLKzphBu1MfZqajKqtWrC3Zbm";

    uint8 validSourcesThreshold;
    uint8 validSignersThreshold;

    uint64 validTimePeriod;
    mapping(address => bool) public trustedSigners;
    mapping(bytes32 => bool) public certificateTrustStore;
    
    mapping(bytes32 => AcelonStructs.PriceEntry) public priceFeeds;

    mapping(address => bool) public owners;
    mapping(address => bool) public proposedOwners;

    constructor(address[] memory _owners, address[] memory _trustedSigners, bytes32[] memory _certificateTrustStore, uint8 _validSignersThreshold, uint8 _validSourcesThreshold, uint64 _validTimePeriod) {
        for(uint8 i = 0; i < _trustedSigners.length; i++) {
            trustedSigners[_trustedSigners[i]] = true;
        }

        for(uint8 i = 0; i < _certificateTrustStore.length; i++) {
            certificateTrustStore[_certificateTrustStore[i]] = true;
        }

        for(uint8 i = 0; i < _owners.length; i++) {
            owners[_owners[i]] = true;
        }

        validSignersThreshold = _validSignersThreshold;
        validSourcesThreshold = _validSourcesThreshold;
        validTimePeriod = _validTimePeriod;
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

    function updateSignersThreshold(uint8 newThreshold) public onlyOwner {
        validSignersThreshold = newThreshold;
    }

    function updateSourcesThreshold(uint8 newThreshold) public onlyOwner {
        validSourcesThreshold = newThreshold;
    }

    function updateValidTimePeriod(uint64 newTimePeriod) public onlyOwner {
        validTimePeriod = newTimePeriod;
    }

    function addTrustedSinger(address signer) public onlyOwner {
        trustedSigners[signer] = true;
    }

    function removeTrustedSinger(address signer) public onlyOwner {
        trustedSigners[signer] = false;
    }

    function addCertificateToTrustStore(bytes32 certificate) public onlyOwner {
        certificateTrustStore[certificate] = true;
    }

    function removeCertificateFromTrustStore(bytes32 certificate) public onlyOwner {
        certificateTrustStore[certificate] = false;
    }

    function recoverSigner(bytes32 _hash, bytes memory _signature) internal pure returns (address) {
        require(_signature.length == 65, "Invalid signature length");

        bytes32 r;
        bytes32 s;
        uint8 v;

        // Extract r, s and v from the signature using assembly
        assembly {
            r := mload(add(_signature, 0x20))
            s := mload(add(_signature, 0x40))
            v := byte(0, mload(add(_signature, 0x60)))
        }

        // ecrecover takes the hash, v, r and s values, and returns the address
        return ecrecover(_hash, v, r, s);
    }

    function updatePriceFeeds(
        bytes[] calldata updateData,
        bytes[][] calldata signature
    ) public override {
        for (uint8 i = 0; i < updateData.length; i++) {
            // 1. check the signature first
            uint8 validSignerCounter = 0;
            for (uint8 k = 0; k < signature[i].length; k++) {
                address recoveredSigner = recoverSigner(keccak256(abi.encodePacked(ACURAST_SIGNATURE_PREFIX, ACURAST_SCRIPT_PREFIX, updateData[i])), signature[i][k]);
                if(trustedSigners[recoveredSigner]){
                    validSignerCounter++;
                    if(validSignerCounter >= validSignersThreshold){
                        break;
                    }
                }
            }
            require(validSignerCounter >= validSignersThreshold,"not enough valid signatures");
            
            // 2. unpack the data
            AcelonStructs.PricePayload memory pricePayload = abi.decode(
                updateData[i],
                (AcelonStructs.PricePayload)
            );
            
            // 3. check certificates
            uint8 validSourcesCounter = 0;
            for (uint8 k = 0; k < pricePayload.certificates.length; k++) {
                if(certificateTrustStore[pricePayload.certificates[k]]){
                    validSourcesCounter++;
                }
                if(validSourcesCounter >= validSourcesThreshold){
                    break;
                }
            }

            require(validSourcesCounter >= validSourcesThreshold,"not enough valid sources");

            // 4. set the new price
            if(priceFeeds[pricePayload.requestHash].timestamp < pricePayload.timestamp){
                AcelonStructs.PriceEntry memory priceEntry = AcelonStructs.PriceEntry(pricePayload.timestamp, pricePayload.prices);
                priceFeeds[pricePayload.requestHash] = priceEntry;
                emit PriceFeedUpdate(pricePayload.requestHash, priceEntry);
            }     
        }
    }

    function priceFeedExists(bytes32 requestHash) public view override returns (bool) {
        return priceFeeds[requestHash].timestamp > 0;
    }
    
    function getValidTimePeriod() public view override returns (uint64){
        return validTimePeriod;
    }

    function getPriceNoOlderThan(bytes32 requestHash, uint64 age) public view override returns (AcelonStructs.PriceEntry memory priceEntry){
        priceEntry = priceFeeds[requestHash];
        require(diff(block.timestamp, priceEntry.timestamp) <= age);
        return priceEntry;
    }

    function getPrice(bytes32 requestHash) public view override returns (AcelonStructs.PriceEntry memory priceEntry){
        return getPriceNoOlderThan(requestHash, getValidTimePeriod());
    }
    
    function diff(uint x, uint y) internal pure returns (uint) {
        if (x > y) {
            return x - y;
        } else {
            return y - x;
        }
    }

}