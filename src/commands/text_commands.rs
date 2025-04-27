use serenity::prelude::*;
use serenity::model::prelude::*;
pub struct TextCommands; 

impl TextCommands {

    pub async fn message( ctx: Context, msg: Message) {
        if msg.content == "!ping"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!dil"
        { 
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("<@{}> has a massive SCHLONG ", 235454585068716032u64)).await
            { 
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!deedee"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("<@{}> will smack you if you step out of line ", 1070730136112078938u64)).await
            { 
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "https://tenor.com/view/betty-boop-whip-gif-27641888").await
            { 
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
}