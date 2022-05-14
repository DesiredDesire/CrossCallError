import { patract, network } from "redspot";
import { AccountId } from "@polkadot/types/interfaces";
import { expect, fromSigner, setupContract } from "../scripts/helpers";
import { setupSystem } from "../scripts/ourDeploy";
import { Signer } from "redspot/types";
import Contract from "@redspot/patract/contract";
const { getSigners, api } = network;
import { consts } from "../scripts/constants";
import { txpayment } from "@polkadot/types/interfaces/definitions";

const DECIMALS = 18;
describe("Deployment", () => {
  let depositerContract: Contract;
  let myPsp22Contract: Contract;
  let users: Signer[];
  let owner: Signer;
  const amount: string = "10000000000000000000000000";

  beforeEach("setup system", async () => {
    users = await getSigners();
    depositerContract = (await setupContract("depositer", "new")).contract;
    const result = await setupContract("psp22_any_mint", "new");
    myPsp22Contract = result.contract;
    owner = result.defaultSigner;
  });

  describe("Tests", async () => {
    it("blabla", async () => {
      await expect(
        myPsp22Contract.query.balanceOf(owner.address)
      ).to.have.output(amount);
      await fromSigner(myPsp22Contract, owner.address).tx.approve(
        depositerContract.address,
        amount
      );
      await expect(
        myPsp22Contract.query.allowance(
          owner.address,
          depositerContract.address
        )
      ).to.have.output(amount);

      await fromSigner(depositerContract, owner.address).tx.depositPsp22(
        myPsp22Contract.address,
        owner.address,
        amount
      );

      await expect(
        myPsp22Contract.query.balanceOf(depositerContract.address)
      ).to.have.output(amount);
    });
  });
});
