use serenity::prelude::*;
use serenity::model::prelude::*;


pub struct ReactionHandler; 


impl ReactionHandler{ 
    pub async fn reaction_add( ctx: Context, reaction: Reaction) {
        println!("Handling reaction: {:?}", reaction);

        if reaction.message_id != 1366013962801647636
        {
            return;
        }

        if let (Some(guild_id), Some(user_id)) = (reaction.guild_id, reaction.user_id) {
            let emoji_str = reaction.emoji.unicode_eq("👍");

            if emoji_str == true
            {
                println!("Assigning role: {:?}", emoji_str);
                let role_id = 1366013701165027408;

                let member = guild_id.member(&ctx.http, user_id).await.unwrap();
                println!("{:?}", member);

                let add = member.add_role(&ctx.http, role_id).await;
                println!("Role added {:?}", add);
            }
        }
    }

    pub async fn reaction_remove( ctx: Context, reaction: Reaction) {
        if reaction.message_id != 1366013962801647636
        {
            return;
        }

        if let (Some(guild_id), Some(user_id)) = (reaction.guild_id, reaction.user_id)
        {
            let emoji_str = reaction.emoji.unicode_eq("👍");

            if emoji_str == true
            {
                println!("Removing role: {:?}", emoji_str);
                let role_id = 1366013701165027408u64;

                let member = guild_id.member(&ctx.http, user_id).await.unwrap();

                let remove = member.remove_role(&ctx.http, role_id).await;
                println!("Role removed: {:?}", remove);
            }

        }

    }
}