import { start } from "solana-bankrun";
import { PublicKey, Transaction, SystemProgram, Connection, Keypair } from "@solana/web3.js";
import { Program } from '@project-serum/anchor';
import * as anchor from '@project-serum/anchor';
import { expect } from "chai";

const PROGRAM_ID = new PublicKey("GtSHNM7qETizmneeqouK5BAfcDAn428SMovfutmqCSZ9");
const TOKEN_PROGRAM_ID = PublicKey.default;

import { IDL } from "../target/types/token_vesting";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";

test("initialize account", async () => {
    const context = await start([], []);
    const client = context.banksClient;
    const payer = context.payer;
    const blockhash = context.lastBlockhash;

    const conntection = new Connection("http://localhost:8899", "confirmed");

    const wallet = new NodeWallet(payer);

    const provider = new anchor.AnchorProvider(conntection, wallet, {commitment: "confirmed"});

    const program = new anchor.Program(IDL, PROGRAM_ID, provider);

    const vestingAccount = Keypair.generate();

    const company_name = "MyCompany";
    const vesting_amount = new anchor.BN(100_000_000);
    const vesting_start_time = new anchor.BN(Math.floor(Date.now() / 1000));
    const vesting_end_time = vesting_start_time.add(new anchor.BN(3600 * 24 * 365));
    const vesting_cliff_period = new anchor.BN(60 * 60 * 24 * 30);

    const tx = await program.methods.initializeAccount(
        company_name,
        vesting_amount,
        vesting_start_time,
        vesting_end_time,
        vesting_cliff_period
    ).accounts({
        tokenVestingAccount: vestingAccount.publicKey,
    }).rpc();

    console.log(`Transaction signature: ${tx}`);
    await conntection.confirmTransaction(tx);
    const balance = await conntection.getBalance(vestingAccount.publicKey);
    console.log(`Vesting Account Balance: ${balance}`);
    expect(balance).greaterThan(0);
});

test("initialize_employee_account", async() => {
    const context = await start([], []);
    const employee = context.payer;
    const connection = new Connection("http://localhost:8899", "confirmed");

    const wallet = new NodeWallet(employee);
    const provider = new anchor.AnchorProvider(connection, wallet, {commitment: "confirmed"});
    const program = new anchor.Program(IDL, PROGRAM_ID, provider);

    const employee_account = Keypair.generate();

    const mint = new PublicKey("So11111111111111111111111111111111111111112");


    const tx = await program.methods.initializeEmployeeAccount().accounts({
        employee: employee.publicKey,
        employeeTokenAccount: employee_account.publicKey,
        mint: mint,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();

    console.log(`Employee account creation transaction signature: ${tx}`);
    await connection.confirmTransaction(tx);

    const balance = await connection.getBalance(employee_account.publicKey);
    console.log(`Employee Account Balance: ${balance}`);
    expect(balance).to.equal(0); 

    const accountInfo = await connection.getAccountInfo(employee_account.publicKey);
    expect(accountInfo).to.not.be.null; 
});

it("claim_tokens", async () => {
    const context = await start([], []);
    const signer = context.payer;
    const connection = new Connection("http://localhost:8899", "confirmed");

    const wallet = new NodeWallet(signer);
    const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
    const program = new anchor.Program(IDL, PROGRAM_ID, provider);

    const employee_account = Keypair.generate();
    const token_vault_account = Keypair.generate();
    const token_vesting_account = Keypair.generate();

    const mint = new PublicKey("So11111111111111111111111111111111111111112");

    const company_name = "ExampleCompany";
    const vesting_amount = new anchor.BN(1_000_000_000); // Example vesting amount
    const vesting_start_time = new anchor.BN(Math.floor(Date.now() / 1000));
    const vesting_end_time = vesting_start_time.add(new anchor.BN(3600 * 24 * 365)); // 1 year
    const vesting_cliff_period = vesting_start_time.add(new anchor.BN(3600 * 24 * 30)); // 1 month

    console.log("Claiming tokens for employee...");

    const tx = await program.methods
        .claimTokens(company_name)
        .accounts({
            signer: signer.publicKey,
            tokenVaultAccount: token_vault_account.publicKey,
            employeeTokenAccount: employee_account.publicKey,
            tokenVestingAccount: token_vesting_account.publicKey,
            mint: mint,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

    console.log(`Transaction signature: ${tx}`);
    await connection.confirmTransaction(tx);

    const employeeBalance = await connection.getTokenAccountBalance(employee_account.publicKey);
    console.log(`Employee Account Token Balance: ${employeeBalance.value.uiAmount}`);
    expect(employeeBalance.value.uiAmount).to.be.greaterThan(0);

    const vaultBalance = await connection.getTokenAccountBalance(token_vault_account.publicKey);
    console.log(`Token Vault Account Balance: ${vaultBalance.value.uiAmount}`);
    expect(vaultBalance.value.uiAmount).to.be.lessThan(vesting_amount.toNumber());
});
