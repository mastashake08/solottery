import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Solottery } from '../target/types/solottery';

describe('solottery', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Solottery as Program<Solottery>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
