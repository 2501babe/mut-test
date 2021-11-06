const anchor = require("@project-serum/anchor");

describe("mut-test", () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.Provider.env());
    const wallet = anchor.getProvider().wallet;
    const program = anchor.workspace.MutTest;
   
    it("Is initialized!", async () => {
        const [boop] = anchor.utils.publicKey.findProgramAddressSync([], program.programId);

        await program.rpc.initialize({
            accounts: {
                user: wallet.publicKey,
                boop: boop,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [wallet.payer],
        });

        // zero
        let boopVal = await program.account.boop.fetch(boop);
        console.log("before touch:", boopVal);
   
        // runs without error
        await program.rpc.touch({
            accounts: {
                boop: boop,
            },
            signers: [wallet.payer],
        });

        // still zero, but touch should have died trying to set 1
        boopVal = await program.account.boop.fetch(boop);
        console.log("after touch:", boopVal);

        // this dies properly without the mut annotation
        await program.rpc.slap({
            accounts: {
                boop: boop,
            },
            signers: [wallet.payer],
        });

        boopVal = await program.account.boop.fetch(boop);
        console.log("after slap:", boopVal);
    });
});
