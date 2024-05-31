use crate::models::{InfractionModel, Punishment, Severity, UserInfractionModel};
use crate::{Context, Error};
use serenity::model::id::UserId;

#[poise::command(
    slash_command,
    prefix_command,
    subcommands("add", "list", "remove", "user"),
    subcommand_required,
    required_permissions = "ADMINISTRATOR",
    category = "Infractions"
)]
pub async fn infractions(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    ephemeral,
    slash_command,
    prefix_command,
    required_permissions = "ADMINISTRATOR",
    guild_only
)]
pub async fn add(
    ctx: Context<'_>,
    id: i32,
    severity: Severity,
    punishment: Punishment,
    duration: i64,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    if let Ok(_) = ctx.data().database.get_infraction(id).await {
        ctx.reply(format!("Infraction ID {id} already exists!"))
            .await?;
        return Ok(());
    }

    if let Ok(infraction) = ctx
        .data()
        .database
        .add_infraction(id, severity, punishment, duration)
        .await
    {
        let data = format_infraction(infraction);
        ctx.reply(format!("Infraction created!\n{data}")).await?;
        return Ok(());
    }

    ctx.reply(format!("Failed to create infraction ID {id}!"))
        .await?;
    Ok(())
}

#[poise::command(
    ephemeral,
    slash_command,
    prefix_command,
    required_permissions = "ADMINISTRATOR",
    guild_only
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    if let Ok(infractions) = ctx.data().database.get_infractions().await {
        let res = infractions
            .iter()
            .map(|i| {
                format!(
                    "- ID: `{}` | Severity: `{:?}` | Punishment: `{:?}` | Duration: `{}`",
                    i.id, i.severity, i.punishment, i.duration
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        ctx.reply(res).await?;
        return Ok(());
    }

    ctx.reply(format!("No infractions found!")).await?;
    Ok(())
}

#[poise::command(
    ephemeral,
    slash_command,
    prefix_command,
    required_permissions = "ADMINISTRATOR",
    guild_only
)]
pub async fn remove(ctx: Context<'_>, id: i32) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    if let Ok(result) = ctx.data().database.remove_infraction(id).await {
        let res = match result.rows_affected() {
            0 => "No infraction removed!".to_owned(),
            1 => "Infraction removed successfully!".to_owned(),
            _ => "Infractions removed successfully!".to_owned(),
        };

        ctx.reply(res).await?;
        return Ok(());
    }

    ctx.reply(format!("Failed to remove infraction!")).await?;
    Ok(())
}

#[poise::command(
    ephemeral,
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "KICK_MEMBERS | BAN_MEMBERS | MODERATE_MEMBERS"
)]
pub async fn user(ctx: Context<'_>, member: UserId) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let user_id = member.get().to_string();

    if let Ok(user_infractions) = sqlx::query_as!(
        UserInfractionModel,
        r#"SELECT * FROM user_infractions WHERE user_id = $1"#,
        user_id,
    )
    .fetch_all(&ctx.data().database.pool)
    .await
    {
        let mut infractions_str = String::new();

        for infraction in user_infractions {
            let formatted = format_user_infraction(infraction);
            infractions_str.push_str(formatted.as_str());
        }

        let vec_pages: Vec<&str> = infractions_str.split("\r\n").collect();
        let pages: &[&str] = vec_pages.as_slice();
        poise::samples::paginate(ctx, pages).await?;
        return Ok(());
    }

    ctx.reply("User has no infractions!").await?;
    Ok(())
}

fn format_infraction(
    InfractionModel {
        id,
        severity,
        punishment,
        duration,
    }: InfractionModel,
) -> String {
    format!(
        "ID: {}\nSeverity: {:?}\nPunishment: {:?}\nDuration: {}\r\n",
        id, severity, punishment, duration
    )
}

fn format_user_infraction(
    UserInfractionModel {
        id,
        user_id,
        infraction_id,
        created_at,
    }: UserInfractionModel,
) -> String {
    format!(
        "<@{}> Case ID: {}\nInfraction ID: {}\nCreated at: {:?}\r\n",
        user_id, id, infraction_id, created_at
    )
}
