use crate::{client::Context, consts::Colors};

use anyhow::Result;
use poise::{
	serenity_prelude::{CreateEmbed, User},
	CreateReply,
};

/// Get someone's profile pic
#[poise::command(context_menu_command = "Get profile picture", slash_command)]
pub async fn pfp(ctx: Context<'_>, user: User) -> Result<()> {
	let url = user
		.avatar_url()
		.unwrap_or_else(|| user.default_avatar_url());
	let embed = CreateEmbed::new()
		.title(user.name)
		.color(Colors::Blue)
		.url(&url)
		.image(&url);
	let reply = CreateReply::default().embed(embed);

	ctx.send(reply).await?;

	Ok(())
}
