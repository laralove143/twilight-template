use anyhow::bail;
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseType},
};

use crate::Context;

pub async fn handle(ctx: Context, interaction: Interaction) -> Result<()> {
    let client = ctx.http.interaction(ctx.application_id);

    let command = match interaction {
        Interaction::ApplicationCommand(cmd) => *cmd,
        _ => {
            bail!("unknown interaction: {interaction:#?}");
        }
    };

    let response_data = match command.data.name.as_str() {
        "" => run_command(ctx, command.data).await?,
        _ => bail!("unknown command: {command:#?}"),
    };

    client
        .create_response(
            command.id,
            &command.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response_data),
            },
        )
        .exec()
        .await?;

    Ok(())
}
