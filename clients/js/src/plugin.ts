import { UmiPlugin } from '@metaplex-foundation/umi';
import { createMplEngraverProgram } from './generated';

export const mplEngraver = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createMplEngraverProgram(), false);
  },
});
