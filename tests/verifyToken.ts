import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { VerifyToken } from '../target/types/verify_token';

describe('verifyToken', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.VerifyToken as Program<VerifyToken>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });

  
});
