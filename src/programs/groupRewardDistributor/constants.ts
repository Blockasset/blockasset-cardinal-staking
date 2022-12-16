import {
  AnchorProvider,
  Program,
  Wallet as AWallet,
} from "@project-serum/anchor";
import type { Wallet } from "@project-serum/anchor/dist/cjs/provider";
import type { ConfirmOptions, Connection } from "@solana/web3.js";
import { Keypair, PublicKey } from "@solana/web3.js";

import type { ParsedIdlAccountData } from "../../accounts";
import * as GROUP_REWARD_DISTRIBUTOR_TYPES from "../../idl/cardinal_group_reward_distributor";

export const GROUP_REWARD_DISTRIBUTOR_ADDRESS = new PublicKey(
  "grwDL1AZiCaBmTQHTQVhX6wxXKowAnisDZxH7866LUL"
);
export const GROUP_REWARD_MANAGER = new PublicKey(
  "crkdpVWjHWdggGgBuSyAqSmZUmAjYLzD435tcLDRLXr"
);

export const GROUP_REWARD_ENTRY_SEED = "group-reward-entry";

export const GROUP_REWARD_COUNTER_SEED = "group-reward-counter";

export const GROUP_REWARD_DISTRIBUTOR_SEED = "group-reward-distributor";

export type GROUP_REWARD_DISTRIBUTOR_PROGRAM =
  GROUP_REWARD_DISTRIBUTOR_TYPES.CardinalGroupRewardDistributor;

export const GROUP_REWARD_DISTRIBUTOR_IDL = GROUP_REWARD_DISTRIBUTOR_TYPES.IDL;

export type GroupRewardEntryData = ParsedIdlAccountData<
  "groupRewardEntry",
  GROUP_REWARD_DISTRIBUTOR_PROGRAM
>;
export type GroupRewardCounterData = ParsedIdlAccountData<
  "groupRewardCounter",
  GROUP_REWARD_DISTRIBUTOR_PROGRAM
>;
export type GroupRewardDistributorData = ParsedIdlAccountData<
  "groupRewardDistributor",
  GROUP_REWARD_DISTRIBUTOR_PROGRAM
>;

export enum GroupRewardDistributorKind {
  Mint = 1,
  Treasury = 2,
}

export enum GroupRewardDistributorMetadataKind {
  NoRestriction = 1,
  UniqueNames = 2,
  UniqueSymbols = 3,
}

export enum GroupRewardDistributorPoolKind {
  NoRestriction = 1,
  AllFromSinglePool = 2,
  EachFromSeparatePool = 3,
}

export const groupRewardDistributorProgram = (
  connection: Connection,
  wallet?: Wallet,
  confirmOptions?: ConfirmOptions
) => {
  return new Program<GROUP_REWARD_DISTRIBUTOR_PROGRAM>(
    GROUP_REWARD_DISTRIBUTOR_IDL,
    GROUP_REWARD_DISTRIBUTOR_ADDRESS,
    new AnchorProvider(
      connection,
      wallet ?? new AWallet(Keypair.generate()),
      confirmOptions ?? {}
    )
  );
};
