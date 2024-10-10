/* eslint-disable no-undef */
// Right click on the script name and hit "Run" to execute
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("AcelonOracle", function () {
  it("test initial value", async function () {
    const AcelonOracle = await ethers.getContractFactory("AcelonOracle");
    const acelonOracle = await AcelonOracle.deploy(["0x5efea821ca12634eef34ad9074086f3db8ef2e95"], ["0x9f828f8e197e0cd081a38be3f5ac5ed91e50a6a676ac2a818ca81c3d0d1aeddd"], 1, 1, 10);
    await acelonOracle.deployed();
    console.log("acelonOracle deployed at:" + acelonOracle.address);
  });

  it("test updating and retrieving updated value", async function () {
    const AcelonOracle = await ethers.getContractFactory("AcelonOracle");
    const acelonOracle = await AcelonOracle.deploy(["0x5efea821ca12634eef34ad9074086f3db8ef2e95"], ["0x9f828f8e197e0cd081a38be3f5ac5ed91e50a6a676ac2a818ca81c3d0d1aeddd"], 1, 1, 10);
    await acelonOracle.deployed();

    const acelonOracle2 = await ethers.getContractAt("AcelonOracle", acelonOracle.address);
    const updatePriceFeeds = await acelonOracle2.updatePriceFeeds(
      ["0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000192479be2e500000000000000000000000000000000000000000000000000000000000000c0c55623dc6bf3806a9e3e11ba957aa37335e154d37dacc702a9ac09f243e7bc8a00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000edd3db170000000000000000000000000000000000000000000000000000000000000000cef27778d3c1f3546d7c49fe18aa7c2559a11171df720b9b4230dc2758bb2603e4795062d13e1ed971c6b6e5699764681e4d090bad39a7ef367cc9cb70565238431257bd5a35d7b4a753f46248c77f4018d7c4192587899ad588ed13529d494849f828f8e197e0cd081a38be3f5ac5ed91e50a6a676ac2a818ca81c3d0d1aedddb9b254d396ed5fda09cd28b30e340d5d0af74027ade9c100a23dd2bb1662407bf48b930f6a5dbad1072bef0d81658ca67019069ed04d2d2feccef4ec90cdf998875f2ab6d013049784ebdca33e5182d8885d81fa9d01ceb44f94175c44a824c17f09db6060e6ab8c7cc96eda8cd93cf629c619e3c522dadc61130530a3681140e949e682ce8ecc2e438b0118f7705377c1297e7a680d352ad98dc2f8d05e157174741668a181b5567f47b82d8a4cd718bd46ae53cd094e049a00cf67db0ae794ff3d152df65fe4b4188e59797e58d0f2c5cf40d7e2eb1dbd48f1dcd97a182550d76729dc646f89f9cf0c4e90d0cc6d9acbd0874304b518da65698edb6d6be65d"],
      [["0x738af8986729d1f9dfe8c3f91c8ba75110b18d5d9ffd9c636d3f8cb9e9774120700082fdd0b84ab189dffded60eef761bf4b7b6604b10c48d0b832722152d2a11b"]]);
    const receipt = await updatePriceFeeds.wait();
    console.log(receipt.events);
  });
});
