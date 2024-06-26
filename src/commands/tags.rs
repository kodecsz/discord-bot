use crate::models::TagModel;
use crate::{Context, Error};
use serenity::model::user::User;

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("add", "edit", "see", "list", "user", "remove"),
    subcommand_required,
    category = "Tags"
)]
pub async fn tag(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(ephemeral, slash_command, prefix_command, guild_only)]
pub async fn add(ctx: Context<'_>, name: String, content: String) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    if let Ok(_) = ctx.data().database.get_tag(&name).await {
        ctx.reply(format!(":warning: Tag `{name}` already exists!"))
            .await?;
        return Ok(());
    }

    if let Ok(tag) = ctx
        .data()
        .database
        .add_tag(&name, &content, ctx.author().id)
        .await
    {
        ctx.reply(format!(
            ":white_check_mark: Tag `{}` created with success!",
            tag.name
        ))
        .await?;
        return Ok(());
    }

    ctx.reply(format!(":x: Cannot create tag {name}!")).await?;
    Ok(())
}

#[poise::command(ephemeral, slash_command, prefix_command, guild_only)]
pub async fn edit(ctx: Context<'_>, name: String, content: String) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let res = match ctx
        .data()
        .database
        .update_tag(&name, &content, ctx.author().id)
        .await
    {
        Err(_) => format!(":x: Tag `{name}` doesn't exist or you're not the owner of it!"),
        Ok(tag) => format!(
            ":white_check_mark: Content of the tag `{}` updated successfully!",
            tag.name
        ),
    };

    ctx.reply(res).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn see(ctx: Context<'_>, name: String) -> Result<(), Error> {
    ctx.defer().await?;

    let res = match ctx.data().database.get_tag(&name).await {
        Err(_) => format!(":x: Tag `{name}` doesn't exists!"),
        Ok(tag) => tag.content,
    };

    ctx.reply(res).await?;
    Ok(())
}

#[poise::command(ephemeral, slash_command, prefix_command, guild_only)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let res = match ctx.data().database.get_all_tags().await {
        Err(_) => format!(":x: Server has no tags!"),
        Ok(tags) => parse_tag_names(&tags),
    };

    ctx.reply(res).await?;
    Ok(())
}

fn parse_tag_names(tags: &[TagModel]) -> String {
    if tags.is_empty() {
        return ":x: No tags!".to_owned();
    }

    let mut names = vec![];
    names.extend(
        tags.iter()
            .map(|t| format!("- `{}` - <@{}>", t.name, t.user_id)),
    );
    names.join("\n")
}

#[poise::command(ephemeral, slash_command, prefix_command, guild_only)]
pub async fn user(ctx: Context<'_>, user: User) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let res = match ctx.data().database.get_user_tags(user.id).await {
        Err(_) => format!(":x: User has no tags!"),
        Ok(tags) => parse_tag_names(&tags),
    };

    ctx.reply(res).await?;
    Ok(())
}

#[poise::command(ephemeral, slash_command, prefix_command, guild_only)]
pub async fn remove(ctx: Context<'_>, name: String) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    if let Err(_) = ctx.data().database.get_tag(&name).await {
        ctx.reply(format!(":x: Tag `{name}` doesn't exists!"))
            .await?;
        return Ok(());
    }

    let res = match ctx
        .data()
        .database
        .remove_tag(&name, ctx.author().id)
        .await
        .unwrap()
        .rows_affected()
    {
        1 => format!(":white_check_mark: Tag `{name}` deleted!"),
        _ => format!(":x: You're not the owner of the tag `{name}`!"),
    };

    ctx.reply(res).await?;
    Ok(())
}
