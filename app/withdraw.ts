import { BN, web3 } from "@coral-xyz/anchor";
import { setup } from "./setup";
import { ComputeBudgetProgram } from "@solana/web3.js";

(async () => {

    const {
        program,
        operator,
        user
    } = await setup();
        console.log("🚀 ~ file: withdraw.ts:12 ~ program:", program.programId)

      
      const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({ 
        microLamports: 
        500000
      });
    const signature = await program.methods
        .withdraw(
            new BN(20000000),
            "1"
    )
        .accounts({
            sender: user.publicKey,
           receiver: user.publicKey,
           operator: operator.publicKey
        })
        
        .signers([ user,operator]).rpc();
    console.log("withdraw: ", signature);
})();
