use revolt_quark::models::emoji::{EmojiParent, PartialEmoji};
use revolt_quark::models::User;
use revolt_quark::{perms, Db, Error, Permission, Ref, Result};
use serde::Deserialize;
use validator::Validate;

use crate::util::regex::RE_EMOJI;

use rocket::serde::json::Json;

#[derive(Validate, Deserialize, JsonSchema)]
pub struct DataEditEmoji {
    #[validate(length(min = 1, max = 32), regex = "RE_EMOJI")]
    name: String,
}

/// # Edit Emoji
///
/// Edit an emoji by its Autumn upload id.
#[openapi(tag = "Emojis")]
#[patch("/emoji/<id>", data = "<data>")]
pub async fn edit_emoji(
    db: &Db,
    user: User,
    id: Ref,
    data: Json<DataEditEmoji>,
) -> Result<Json<PartialEmoji>> {
    let data = data.into_inner();
    data.validate()
        .map_err(|error| Error::FailedValidation { error })?;

    if user.bot.is_some() {
        return Err(Error::IsBot);
    }

    // Fetch the emoji
    let emoji = id.as_emoji(db).await?;

    // If we uploaded the emoji, then we have permission to edit it
    if emoji.creator_id != user.id {
        // Otherwise, validate we have permission to edit from parent
        match &emoji.parent {
            EmojiParent::Server { id } => {
                let server = db.fetch_server(id).await?;

                // Check for permission
                perms(&user)
                    .server(&server)
                    .throw_permission(db, Permission::ManageCustomisation)
                    .await?;
            }
            EmojiParent::Detached => return Err(Error::InvalidOperation),
        };
    }

    let partial = PartialEmoji {
        id: id.id,
        name: data.name,
    };

    emoji.update_emoji(db, &partial).await?;
    Ok(Json(partial))
}
