import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolQuest } from "../target/types/sol_quest";

describe("sol-quest", () => {
  var provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolQuest as Program<SolQuest>;
  const [adminAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("Admin"), provider.publicKey.toBuffer()], program.programId);

  it("Initialize Admin!", async () =>
  {

    await program.methods
      .initializeAdmin()
      .accounts({
        signer: provider.publicKey,
        admin: adminAccountPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    
    const adminAccount = await program.account.admin.fetch(adminAccountPDA);
    
    console.log(adminAccount.authority.toString());
    console.log(adminAccount.matesSubmitted);
  });

  it("Initialize Mate!", async () =>
  {
    const [mateAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("Mate"), provider.publicKey.toBuffer()], program.programId);

    await program.methods
      .initializeUser(new anchor.web3.PublicKey("4rByWqQnzNL3Zrpk6sgF22SwZCCzqc7oNP2HGHUK2iu3"))
      .accounts({
        signer: provider.publicKey,
        user: mateAccountPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    
    const mateAccount = await program.account.mate.fetch(mateAccountPDA);
    
    console.log(mateAccount.authority.toString());
    console.log(mateAccount.mateNft.toString());
    console.log(mateAccount.mateJoinedDate.toNumber());
    console.log(mateAccount.mateRole);
    console.log(mateAccount.questCompletedByMate);
  });


  it("Add Mate Social!", async () =>
  {
    const [mateAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("Mate"), provider.publicKey.toBuffer()], program.programId);

    await program.methods
      .addMateSocial([{ socialName: "Twitter", socialLink: "@memxor_"}, { socialName: "Instagram", socialLink: "@memxor"},{ socialName: "Twitter", socialLink: "@MEMXOR"}])
      .accounts({
        signer: provider.publicKey,
        user: mateAccountPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
    
    const mateAccount = await program.account.mate.fetch(mateAccountPDA);
    
    console.log(mateAccount.socials.length);
    console.log(mateAccount.socials[0]);
    console.log(mateAccount.socials[1]);
  });

  it("Add mate quest!", async () =>
  {
    const [mateAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("Mate"), provider.publicKey.toBuffer()], program.programId);

    await program.methods
    .addCompletedQuest(1, "github.com/memxor/example", "dfh9pauwfbhiuohbfiuPHNOUIHNOIPHFNDOSIFNOnsonfosinfo[s'fnosifnsofnin'fnd")
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
  
    await program.methods
    .addCompletedQuest(2, "github.com/memxor/example", "dfh9pauwfbhiuohbfiuPHNOUIHNOIPHFNDOSIFNOnsonfosinfo[s'fnosifnsofnin'fnd")
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
  
    await program.methods
    .addCompletedQuest(3, "github.com/memxor/example", "dfh9pauwfbhiuohbfiuPHNOUIHNOIPHFNDOSIFNOnsonfosinfo[s'fnosifnsofnin'fnd")
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
  
    await program.methods
    .addCompletedQuest(4, "github.com/memxor/example", "dfh9pauwfbhiuohbfiuPHNOUIHNOIPHFNDOSIFNOnsonfosinfo[s'fnosifnsofnin'fnd")
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
  
    await program.methods
    .addCompletedQuest(5, "github.com/memxor/example", "dfh9pauwfbhiuohbfiuPHNOUIHNOIPHFNDOSIFNOnsonfosinfo[s'fnosifnsofnin'fnd")
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
    
    const mateAccount = await program.account.mate.fetch(mateAccountPDA);
    const adminAccount = await program.account.admin.fetch(adminAccountPDA);
    
    console.log(mateAccount.authority.toString());
    console.log(mateAccount.mateNft.toString());
    console.log(mateAccount.mateJoinedDate.toNumber());
    console.log(mateAccount.mateRole);
    console.log(mateAccount.questCompletedByMate);

    console.log(adminAccount.matesSubmitted);
  });

  it("Approve mate quest!", async () => {
    const [mateAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("Mate"), provider.publicKey.toBuffer()], program.programId);

    await program.methods
    .approveUserQuest(1)
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
    
    await program.methods
    .approveUserQuest(2)
    .accounts({
      signer: provider.publicKey,
      user: mateAccountPDA,
      admin: adminAccountPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
    
    const mateAccount = await program.account.mate.fetch(mateAccountPDA);
    const adminAccount = await program.account.admin.fetch(adminAccountPDA);

    console.log(mateAccount.questCompletedByMate);
    console.log(adminAccount.matesSubmitted);
  });

});
