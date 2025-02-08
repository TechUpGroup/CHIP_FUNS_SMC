import * as anchor from "@coral-xyz/anchor";
import { ChipFunSc } from "../target/types/chip_fun_sc";

export async function setup() {
    const connection = new anchor.web3.Connection(
        anchor.web3.clusterApiUrl("devnet")
    );
    const idl = require("../target/idl/chip_fun_sc.json");
    const path_authority_key = "devnet.json";
    const operator = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(require(path_authority_key))
    ) as anchor.web3.Keypair;
    console.log("🚀 ~ file: setup.ts:13 ~ setup ~ operator:", operator.publicKey)
    const balance =await  connection.getBalance(operator.publicKey);
    console.log("🚀 ~ file: setup.ts:16 ~ setup ~ balance:", balance)
    const path_user_key = "dev2.json";
    const user = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(require(path_user_key))
    ) as anchor.web3.Keypair;
    console.log("🚀 ~ file: setup.ts:21 ~ setup ~ user:", user.publicKey)

    const wallet = new anchor.Wallet(user);
    const provider = new anchor.AnchorProvider(connection, wallet, {
        commitment: "confirmed",
    });


    const program = new anchor.Program(
        idl as ChipFunSc,
        provider
    );


    return {
        program,
        operator,
        connection,
        user
    };
}
