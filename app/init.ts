import { web3 } from "@coral-xyz/anchor";
import { setup } from "./setup";
import { ComputeBudgetProgram } from "@solana/web3.js";

(async () => {

    const {
        program,
        operator,
        user
    } = await setup();
        console.log("🚀 ~ file: init.ts:12 ~ program:", program.programId)

      
    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({ 
        units: 500000 
      });
      
      const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({ 
        microLamports: 10000000
      });
    const signature = await program.methods
        .initialize(
    )
        .accounts({
            authority: operator.publicKey,
            operator: operator.publicKey,
            mint: new web3.PublicKey("HiEWJhkSTTQe6Uzjv2APNG659xygGKgsTTEbsZzA2Aq"),
            
        })
        .preInstructions([
            modifyComputeUnits,
            addPriorityFee
        ])
        .signers([operator]).rpc();
    console.log("init: ", signature);
})();
