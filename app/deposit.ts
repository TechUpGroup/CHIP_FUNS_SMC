import { BN, web3 } from "@coral-xyz/anchor";
import { setup } from "./setup";
import { ComputeBudgetProgram } from "@solana/web3.js";

(async () => {

    const {
        program,
        operator,
        user
    } = await setup();

      
    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({ 
        units: 1000000 
      });
      
      const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({ 
        microLamports: 1000000000
      });
    const signature = await program.methods
        .deposit(
            new BN(100000000),
            "1"
    )
        .accounts({
           depositor: operator.publicKey,
           
        })
        .preInstructions([
            modifyComputeUnits,
            addPriorityFee
        ])
        .signers([operator]).rpc({skipPreflight: true});
    console.log("deposit: ", signature);
})();
