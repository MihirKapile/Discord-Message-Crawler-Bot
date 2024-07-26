use serenity::{
    async_trait, client::bridge::gateway::GatewayIntents, model::{channel::Message, gateway::Ready, prelude::{Embed, EmbedField}}, prelude::*, utils::Colour
};

use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let specific_user_id: u64 = 322192154833321995; // Specific user ID

        if msg.author.id.0 == specific_user_id {
            let message_content = msg.content.to_lowercase();
            let keyword = "bruh";
            if message_content.starts_with(keyword) {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Sera you a dumb dumb").await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }

        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
            return;
        }

        if msg.content.starts_with("!seraskip") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() != 3 {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Usage: !seraskip <skip> <take>").await {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }
        
            let user_id = "322192154833321995";
            let skip: usize = match args[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Please provide a valid number for skip.").await {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                }
            };
        
            let take: usize = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Please provide a valid number for take.").await {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                }
            };
        
            if let Err(why) = seraskip(&ctx, &msg, user_id, skip, take).await {
                println!("Error creating embed: {:?}", why);
            }
        }
        

        if msg.content.starts_with("!crawl") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() != 3 {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Usage: !crawl <user_id> <number_of_messages>").await {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }

            let user_id = args[1];
            let number_of_messages: usize = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Please provide a valid number of messages.").await {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                }
            };

            if let Err(why) = crawl_messages(&ctx, &msg, user_id, number_of_messages).await {
                println!("Error crawling messages: {:?}", why);
            }
        }
        if msg.content.starts_with("!seraalert") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() != 2 {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Usage: !seraalert <number_of_messages>").await {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }

            let user_id = "322192154833321995";
            let number_of_messages: usize = match args[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Please provide a valid number of messages.").await {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                }
            };

            if let Err(why) = seraalert(&ctx, &msg, user_id, number_of_messages).await {
                println!("Error creating embed: {:?}", why);
            }
        }
    

    }

    

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn seraskip(ctx: &Context, msg: &Message, user_id: &str, skip: usize, take: usize) -> serenity::Result<()> {
    let channel_id = msg.channel_id;
    let channel = channel_id.to_channel(&ctx.http).await?.guild().unwrap();
    let mut messages = Vec::new();
    let mut last_message_id = None;
    let mut user_messages = Vec::new();

    while user_messages.len() < skip + take {
        let mut msgs = if let Some(last_message_id) = last_message_id {
            channel.messages(&ctx.http, |retriever| retriever.before(last_message_id).limit(100)).await?
        } else {
            channel.messages(&ctx.http, |retriever| retriever.limit(100)).await?
        };

        if msgs.is_empty() {
            break;
        }

        last_message_id = msgs.last().map(|m| m.id);

        for message in &msgs {
            if message.author.id.to_string() == user_id {
                user_messages.push(message.clone());
                if user_messages.len() >= skip + take {
                    break;
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    if user_messages.len() > skip {
        messages = user_messages.into_iter().skip(skip).take(take).collect();
    }

    println!("Found {} messages after skipping {} messages", messages.len(), skip);

    let mut embed = serenity::builder::CreateEmbed::default();
    embed.title("User Messages")
         .description(format!("Messages from user after skipping last {} messages", skip))
         .colour(Colour::DARK_GREEN);

    for message in &messages {
        let content = if message.content.is_empty() {
            "No content".to_string()
        } else {
            message.content.clone()
        };

        println!("Adding field for message from {}", message.author.name);
        println!("Content: {}", content);

        embed.field(
            format!("Message from {}", message.author.name),
            content,
            false,
        );
    }

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
        println!("Error sending embed: {:?}", why);
    } else {
        println!("Embed sent successfully!");
    }

    Ok(())
}


async fn seraalert(ctx: &Context, msg: &Message, user_id: &str, number_of_messages: usize) -> serenity::Result<()> {
    let channel_id = msg.channel_id;
    let channel = channel_id.to_channel(&ctx.http).await?.guild().unwrap();
    let mut messages = Vec::new();
    let mut last_message_id = None;

    while messages.len() < number_of_messages {
        let mut _msgs = if let Some(last_message_id) = last_message_id {
            channel.messages(&ctx.http, |retriever| retriever.before(last_message_id).limit(100)).await?
        } else {
            channel.messages(&ctx.http, |retriever| retriever.limit(100)).await?
        };

        if _msgs.is_empty() {
            break;
        }

        last_message_id = _msgs.last().map(|m| m.id);

        for message in &_msgs {
            if message.author.id.to_string() == user_id {
                messages.push(message.clone());
                println!("Found {} messages", messages.len());
                if messages.len() >= number_of_messages {
                    break;
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    println!("Found {} messages", messages.len());

    let mut embed = serenity::builder::CreateEmbed::default();
    embed.title("User Messages")
         .description(format!("Last {} messages from user", number_of_messages))
         .colour(Colour::DARK_GREEN);

    for message in messages.iter().take(number_of_messages) {
        let content = if message.content.is_empty() {
            "No content".to_string()
        } else {
            message.content.clone()
        };
    
        println!("Adding field for message from {}", message.author.name);
        println!("Content: {}", content);
    
        embed.field(
            format!("Message from {}", message.author.name),
            content,
            false,
        );
    }
    

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
        println!("Error sending embed: {:?}", why);
    } else {
        println!("Embed sent successfully!");
    }

    Ok(())
}

async fn crawl_messages(ctx: &Context, msg: &Message, user_id: &str, number_of_messages: usize) -> serenity::Result<()> {
    let channel_id = msg.channel_id;
    let channel = channel_id.to_channel(&ctx.http).await?.guild().unwrap();
    let mut messages = Vec::new();
    let mut last_message_id = None;

    while messages.len() < number_of_messages {
        let mut msgs = if let Some(last_message_id) = last_message_id {
            channel.messages(&ctx.http, |retriever| retriever.before(last_message_id).limit(100)).await?
        } else {
            channel.messages(&ctx.http, |retriever| retriever.limit(100)).await?
        };

        if msgs.is_empty() {
            break;
        }

        last_message_id = msgs.last().map(|m| m.id);

        for message in &msgs {
            if message.author.id.to_string() == user_id {
                messages.push(message.clone());
                if messages.len() >= number_of_messages {
                    break;
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    for message in messages.iter().take(number_of_messages) {
        if let Err(why) = msg.channel_id.say(&ctx.http, format!("{}: {}", message.author.name, message.content)).await {
            println!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = "MTI2NjE3MzIyNDQ1MTA1MTYwMg.GuR61f.f3C83pbQR-aWLc0Ar1NJmglpcD--HYH2M17R6k";


    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
