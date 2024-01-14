# solQuest Code Explanation

### Constants
Constants defining the number of completed quests required for different levels and seed values.
```
pub const QUEST_REQUIRED_FOR_SILVER: i8 = 5;
pub const QUEST_REQUIRED_FOR_GOLD: i8 = 10;
pub const QUEST_REQUIRED_FOR_PLATINUM: i8 = 15;
pub const MATE_SEED: &[u8; 4] = b"Mate";
pub const ADMIN_SEED: &[u8; 5] = b"Admin";
```

### Function: initialize_admin
This function initializes the admin account. It checks if the provided signer's key matches a predefined key. If the check passes, it sets the admin's authority to the signer's key.
```
pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
    assert!(ctx.accounts.signer.key().to_string() == "3jyefQuStD7c2McYUKyGT4uwFKMVTm1sJzHQZo8JbQvi");
    ctx.accounts.admin.authority = ctx.accounts.signer.key();
    Ok(())
}
```

### Function: initialize_user
This function initializes a new user account. It sets the user's authority to the signer's key. The provided NFT mint is associated with the user. The user's joined date is set to the current timestamp. The user starts with no completed quests, a Bronze role, and an empty list of socials.
```
pub fn initialize_user(ctx: Context<InitializeUser>, nft_mint: Pubkey) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.authority = ctx.accounts.signer.key();
    user.mate_nft = nft_mint;
    user.mate_joined_date = Clock::get().unwrap().unix_timestamp as i64;
    user.quest_completed_by_mate = Vec::new();
    user.mate_role = MateRole::Bronze;
    user.socials = Vec::new();
    Ok(())
}
```

### Function: add_completed_quest
This function adds a completed quest to the user's account. It creates a `Quest` struct and pushes it to the user's list of completed quests. The function checks if the number of completed quests qualifies the user for a new role and updates it. If the user is not in the admin's submitted list, it adds them.
```
pub fn add_completed_quest(ctx: Context<AddCompletedQuest>, id: i8, deployed_url: String, transaction: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let admin_user = &mut ctx.accounts.admin;

    let quest = Quest {
        id,
        deployed_url,
        transaction,
        updated_time: Clock::get().unwrap().unix_timestamp as i64,
        status: Status::SUBMITTED
    };

    user.quest_completed_by_mate.push(quest);

    // Check if the completed quests qualify for a new role
    // Update the user's role accordingly
    if user.quest_completed_by_mate.len() as i8 >= QUEST_REQUIRED_FOR_PLATINUM {
        user.mate_role = MateRole::Platinum;
    } else if user.quest_completed_by_mate.len() as i8 >= QUEST_REQUIRED_FOR_GOLD {
        user.mate_role = MateRole::Gold;
    } else if user.quest_completed_by_mate.len() as i8 >= QUEST_REQUIRED_FOR_SILVER {
        user.mate_role = MateRole::Silver;
    }

    // If the user is not already in the admin's submitted list, add them
    if !admin_user.mates_submitted.contains(&user.authority) {
        admin_user.mates_submitted.push(user.authority.clone())
    }

    Ok(())
}
```

### Function: add_mate_social
This function adds social links to a user's account. It iterates through the provided socials and checks if each already exists for the user. If a social link already exists, it updates the link. Otherwise, it adds a new social link.
```
pub fn add_mate_social(ctx: Context<AddMateSocial>, socials: Vec<Social>) -> Result<()> {
    let user = &mut ctx.accounts.user;

    for social in socials {
        let mut social_already_exists = false;
        for existing_socials in &mut user.socials {
            if social.social_name == existing_socials.social_name {
                existing_socials.social_link = social.social_link.clone();
                social_already_exists = true;
            }
        }

        if !social_already_exists {
            user.socials.push(social);
        }
    }

    Ok(())
}
```

### Function: approve_user_quest
This function approves a user's quest. It updates the quest status to `ACCEPTED` based on the provided `quest_id`. It checks if the user has any remaining submitted quests. If not, it removes them from the admin's submitted list.
```
pub fn approve_user_quest(ctx: Context<ApproveMateQuestStatus>, quest_id: i8) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let admin_user = &mut ctx.accounts.admin;

    let mut mate_has_no_remaining_submitted_quest = true;

    for quest in &mut user.quest_completed_by_mate {
        if quest.id == quest_id {
            quest.status = Status::ACCEPTED;
        }

        if quest.status == Status::SUBMITTED {
            mate_has_no_remaining_submitted_quest = false;
        }
    }

    // If the user has no remaining submitted quests, remove them from admin's list
    if mate_has_no_remaining_submitted_quest {
        let index = admin_user.mates_submitted.iter().position(|x| *x == user.authority.key()).unwrap();
        admin_user.mates_submitted.remove(index);
    }

    Ok(())
}
```

### Struct: Admin
`Admin` is an account struct representing the admin of the system. It has two fields: `authority` (a public key) and `mates_submitted` (a vector of public keys). The `LEN` constant is used to define the space required for an admin account.
```
#[account]
pub struct Admin {
    pub authority: Pubkey,
    pub mates_submitted: Vec<Pubkey>
}

impl Admin {
    pub const LEN: usize = 8 + 32 + 4;
}
```

### Struct: Mate
`Mate` is an account struct representing a user or mate in the system. It has several fields including `authority` (public key), `mate_nft` (NFT mint associated with the mate), `mate_joined_date` (timestamp), `quest_completed_by_mate` (vector of completed quests), `mate_role` (enumeration representing the mate's role), and `socials` (vector of social links). The `LEN` constant defines the space required for a mate account.
```
#[account]
pub struct Mate {
    pub authority: Pubkey,
    pub mate_nft: Pubkey,
    pub mate_joined_date: i64,
    pub quest_completed_by_mate: Vec<Quest>,
    pub mate_role: MateRole,
    pub socials: Vec<Social>
}

impl Mate {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 4 + 1 + 4 + 8;
}
```

### Struct: Social
`Social` is a simple struct representing a social link associated with a mate. It has two fields: `social_name` (the name of the social platform) and `social_link` (the link to the mate's profile on that platform).
```
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Social {
    pub social_name: String,
    pub social_link: String
}

impl Social
{
    fn get_social_length(socials: &Vec<Social>) -> usize
    {
        let size: usize = socials.iter().map(|x| x.social_name.len() + x.social_link.len() + 8).into_iter().sum();
        size
    }
}
```

### Struct: Quest
`Quest` is a struct representing a completed quest by a mate.
It has fields such as `id` (unique identifier), `deployed_url` (URL where the quest was deployed), `transaction` (transaction related to the quest), `updated_time` (timestamp when the quest was updated), and `status` (enumeration representing the quest status).
```
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Quest {
    pub id: i8,
    pub deployed_url: String,
    pub transaction: String,
    updated_time: i64,
    status: Status
}

impl Quest
{
    fn get_quests_length(quests: &Vec<Quest>) -> usize
    {
        let size: usize = quests.iter().map(|x| x.deployed_url.len() + x.transaction.len() + 18).into_iter().sum();
        size
    }

    fn get_quest_length(deployed_url: &String, transaction: &String) -> usize
    {
        let size: usize = deployed_url.len() + transaction.len() + 18;
        size
    }
}
```

### Enum: MateRole
`MateRole` is an enumeration representing the role of a mate in the system. It can be one of four roles: `Bronze`, `Silver`, `Gold`, or `Platinum`.
```
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MateRole {
    Bronze,
    Silver,
    Gold,
    Platinum
}
```

### Enum: Status
`Status` is an enumeration representing the status of a quest. It can be either `SUBMITTED` or `ACCEPTED`.
```
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Status {
    SUBMITTED,
    ACCEPTED
}
```
