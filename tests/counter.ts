import * as anchor from "@coral-xyz/anchor";

describe("counter_program", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterProgram;

  const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Initialising the counter", async () => {
    await program.methods
      .initialize()
      .accounts({
        counterAccount: counterPda,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const acc = await program.account.counterAccount.fetch(counterPda);
    console.log("Counter value after init:", acc.count.toString());

  });

  it("Incrementing the counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counterAccount: counterPda,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const acc = await program.account.counterAccount.fetch(counterPda);
    console.log("Counter after increment:", acc.count.toString());
  })
})
