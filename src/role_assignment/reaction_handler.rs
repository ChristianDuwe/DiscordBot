use serenity::prelude::*;
use serenity::model::prelude::*;


pub struct ReactionHandler; 


impl ReactionHandler{ 
    pub async fn reaction_add( ctx: Context, reaction: Reaction) {
        if reaction.message_id != 1398319986111549586
        {
            return;
        }

        if let (Some(guild_id), Some(user_id)) = (reaction.guild_id, reaction.user_id) 
        {
            let (custom_id, custom_name) = match &reaction.emoji
            { 
                ReactionType::Custom {id, name, .. } => (Some(id.get()), name.as_deref()),
                _ => (None, None),
            };

            match (custom_id, custom_name)
            {
                (Some(1195363888254955682), Some("LWKekW")) => 
                {
                    Self::add_role(ctx, guild_id, user_id, 1087414816068620358.into()).await;
                }
                (Some(1105174564390060052), Some("pepeLove")) => 
                {
                    Self::add_role(ctx, guild_id, user_id, 1087414434441461840.into()).await;
                }
                _ => {
                    println!("No match");
                }
            }   
        }
    }

    pub async fn reaction_remove( ctx: Context, reaction: Reaction) {
        if reaction.message_id != 1398319986111549586
        {
            return;
        }

        if let (Some(guild_id), Some(user_id)) = (reaction.guild_id, reaction.user_id)
        { 
            let (custom_id, custom_name) = match &reaction.emoji
            { 
                ReactionType::Custom {id, name, .. } => (Some(id.get()), name.as_deref()),
                _ => (None, None),
            };

            match (custom_id, custom_name)
            { 
                (Some(1195363888254955682), Some("LWKekW")) => 
                {
                    Self::remove_role(ctx, guild_id, user_id, 1087414816068620358.into()).await;
                }
                (Some(1105174564390060052), Some("pepeLove")) => 
                {
                    Self::remove_role(ctx, guild_id, user_id, 1087414434441461840.into()).await;
                }
                _ => {
                    println!("No match");
                }
            }
        }
    }

    async fn add_role(ctx: Context, guild_id: GuildId, user_id: UserId, role_id: RoleId) 
    {
        let member = guild_id.member(&ctx.http, user_id).await.unwrap();
        println!("{:?}", member);

        let add = member.add_role(&ctx.http, role_id).await;
        println!("Role added {:?}", add);
    }

    async fn remove_role(ctx: Context, guild_id: GuildId, user_id: UserId, role_id: RoleId) 
    { 
        let member = guild_id.member(&ctx.http, user_id).await.unwrap();
        println!("{:?}", member);

        let remove = member.remove_role(&ctx.http, role_id).await;
        println!("Role removed {:?}", remove);
    }
}