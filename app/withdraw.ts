import { BN, web3 } from "@coral-xyz/anchor";
import { setup } from "./setup";
import { ComputeBudgetProgram } from "@solana/web3.js";

(async () => {

    const {
        program,
        operator,
        user
    } = await setup();

      
      const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({ 
        microLamports: 
        500000
      });
    const signature = await program.methods
        .withdraw(
            new BN(50000000),
            "1"
    )
        .accounts({
           receiver: user.publicKey,
           sender: user.publicKey,
           
        })
        .preInstructions([
            addPriorityFee
        ])
        .signers([operator,user]).rpc({skipPreflight: true});
    console.log("init: ", signature);
})();
