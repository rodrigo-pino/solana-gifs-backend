const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() => {
    console.log("Starting tests ... Ou Mama");

    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Solanagifs;

    const baseAccount = anchor.web3.Keypair.generate();

    const tx = await program.rpc.initialize({
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
    });

    console.log("Your transaction signature", tx);

    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);

    await program.rpc.addGif(
        "https://media.giphy.com/media/Vyh8EchrZeMaSteCJP/giphy.gif",
        provider.wallet.publicKey,{
        accounts: {
            baseAccount: baseAccount.publicKey,
        },
    });

    await program.rpc.addGif(
        "https://media.giphy.com/media/Vyh8EchrZeMaSteCJP/giphy.gif2",
        provider.wallet.publicKey, {
        accounts: {
            baseAccount: baseAccount.publicKey,
        },
    });

    await program.rpc.upvoteGif(
        1,{
        accounts: {
            baseAccount: baseAccount.publicKey,
        }
    })

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("Gif Count", account.totalGifs.toString());
    console.log("GIF List", account.gifList);

    console.log("A gif upvote", account.gifList[0].gifUpvotes)

    console.log("Sending SOL")
    account = await program.rpc.sendSol(
            1000,
            {
            accounts: {
                from: provider.wallet.publicKey,
                to: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            }
        }
    );
    console.log("Sol sended");

}

const runMain = async () => {
    try {
        await main();
        process.exit(0);
    } catch (error) {
        console.error(error);
        process.exit(1)
    }
};

runMain();
