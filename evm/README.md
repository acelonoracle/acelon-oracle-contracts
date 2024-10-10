# Acelon Oracle EVM smart contract

This smart contract implements the logic to receive price updated from [Acelon](https://acelon.io/) oracles.

## Overview

This workspace contains 3 directories:

1. 'contracts': Holds three contracts with increasing levels of complexity.
2. 'scripts': Contains four typescript files to deploy a contract. It is explained below.
3. 'tests': Contains one Solidity test file for 'Ballot' contract & one JS test file for 'Storage' contract.

### Scripts

The 'scripts' folder has four typescript files which help to deploy the 'Storage' contract using 'web3.js' and 'ethers.js' libraries.

For the deployment of any other contract, just update the contract's name from 'Storage' to the desired contract and provide constructor arguments accordingly
in the file `deploy_with_ethers.ts` or  `deploy_with_web3.ts`

In the 'tests' folder there is a script containing Mocha-Chai unit tests for 'Storage' contract.

To run a script, right click on file name in the file explorer and click 'Run'. Remember, Solidity file must already be compiled.
Output from script will appear in remix terminal.

Please note, require/import is supported in a limited manner for Remix supported modules.
For now, modules supported by Remix are ethers, web3, swarmgw, chai, multihashes, remix and hardhat only for hardhat.ethers object/plugin.
For unsupported modules, an error like this will be thrown: '<module_name> module require is not supported by Remix IDE' will be shown.

## Usage

The core functionality the contract offers is the `updatePriceFeeds` call, which takes as input a list of updates and corresponding signatures that have been produced by the Acelon oracles.

See the test in [acelon.test.js](tests/acelon.test.js) for an example.

The contract also stores a list of valid signers (Oracle addresses) and certificate hashes, only prices provided by the those Oracles (singers) and certificates will be accepted.
