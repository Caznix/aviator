
use poise::serenity_prelude as serenity;
use ::serenity::json::NULL;
use tokio;
use util::Verifiy;


use clap::Parser;

use sysinfo::System as OtherSystem;
// replace this later

pub mod util;
#[derive(Parser)]
#[command(version, about, long_about = None)]

struct Cli {

    #[arg(short, long,)]
    debug: Option<bool>

}
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    println!("{u} ran /age. responding with -> {response}");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn quit(
    ctx: Context<'_>,
    #[description = "Make the bot quit"] reason: Option<String>
) -> Result<(), Error> {
    let can_quit: String = std::env::var("AVIATOR_DEBUG").expect("");
    let default: String = "No reason specified".to_string();
    let r: String = reason.unwrap_or_else(||default);
    if can_quit == "TRUE" && can_quit != NULL {
        let text: String = format!("Terminating bot. Quitting for: {r}");
        ctx.say(text).await?;
        std::process::exit(0);
        } else {
            let text: String = format!("Quit command not enabled. add AVIATOR_MODE=TRUE before your startup command");
            ctx.say(text).await?;
            Ok(())
        }
    }
#[poise::command(slash_command, prefix_command)]
async fn get_env(
    ctx: Context<'_>,
    #[description = "Make the bot quit"] verbose: Option<bool>
) -> Result<(),Error> {
    let host_name =  hostname::get()?;
    let can_quit: String = std::env::var("AVIATOR_DEBUG").expect("");
    let mut sys = OtherSystem::new_all();
    let mem_usage: u64 = sys.used_memory()/1000000;
    let r: String = format!("Currently running on:{:?}\n
AVIATOR_DEBUG=={:?}\n
Current Memory usage is {:?}MB",host_name,can_quit,mem_usage);
    ctx.reply(r).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn echo(
    ctx: Context<'_>,
    #[description = "Make the bot quit"] input_msg: String
    ) -> Result<(), Error> {
    let input_msg = format!("{:#?}",input_msg).verify().unwrap();
    ctx.reply(input_msg).await?;
    Ok(())
}

    fn main() {
        let cli = Cli::parse();
        match cli.debug {
            Some(..) => {
                std::env::set_var("AVIATOR_DEBUG", "TRUE");
                print!("Debug mode enabled")
            },
            None => {
                std::env::set_var("AVIATOR_DEBUG", "FALSE")
            }
        }
        main_run()
    }


#[tokio::main]
async fn main_run() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(),quit(),get_env(),echo()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}