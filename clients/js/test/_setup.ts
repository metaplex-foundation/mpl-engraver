/* eslint-disable import/no-extraneous-dependencies */
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import { mplEngraver } from '../src';

export const createUmi = async () =>
  (await basecreateUmi()).use(mplEngraver());
