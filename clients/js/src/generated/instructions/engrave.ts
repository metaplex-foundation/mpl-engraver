/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  struct,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  EngraveTarget,
  EngraveTargetArgs,
  getEngraveTargetSerializer,
} from '../types';

// Accounts.
export type EngraveInstructionAccounts = {
  /** NFT update authority */
  authority?: Signer;
  /** NFT mint account */
  mint: PublicKey | Pda;
  /** NFT token account */
  token: PublicKey | Pda;
  /** NFT metadata account */
  metadata: PublicKey | Pda;
  /** NFT edition account */
  edition: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type EngraveInstructionData = {
  discriminator: number;
  target: EngraveTarget;
  data: Uint8Array;
};

export type EngraveInstructionDataArgs = {
  target: EngraveTargetArgs;
  data: Uint8Array;
};

export function getEngraveInstructionDataSerializer(): Serializer<
  EngraveInstructionDataArgs,
  EngraveInstructionData
> {
  return mapSerializer<EngraveInstructionDataArgs, any, EngraveInstructionData>(
    struct<EngraveInstructionData>(
      [
        ['discriminator', u8()],
        ['target', getEngraveTargetSerializer()],
        ['data', bytes({ size: u32() })],
      ],
      { description: 'EngraveInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<EngraveInstructionDataArgs, EngraveInstructionData>;
}

// Args.
export type EngraveInstructionArgs = EngraveInstructionDataArgs;

// Instruction.
export function engrave(
  context: Pick<Context, 'identity' | 'programs'>,
  input: EngraveInstructionAccounts & EngraveInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplEngraver',
    'ENGRVY4DL6uKDnNS91hCkJMwzTfcofYpkZH8zsgJfzA3'
  );

  // Accounts.
  const resolvedAccounts: ResolvedAccountsWithIndices = {
    authority: { index: 0, isWritable: true, value: input.authority ?? null },
    mint: { index: 1, isWritable: true, value: input.mint ?? null },
    token: { index: 2, isWritable: true, value: input.token ?? null },
    metadata: { index: 3, isWritable: true, value: input.metadata ?? null },
    edition: { index: 4, isWritable: true, value: input.edition ?? null },
    systemProgram: {
      index: 5,
      isWritable: false,
      value: input.systemProgram ?? null,
    },
  };

  // Arguments.
  const resolvedArgs: EngraveInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getEngraveInstructionDataSerializer().serialize(
    resolvedArgs as EngraveInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
