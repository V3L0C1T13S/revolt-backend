use revolt_quark::{
    models::{user::UserProfile, User},
    Result,
};

use rocket::serde::json::Json;

/// # Fetch Self Profile
///
/// Retrieve your profile.
#[openapi(tag = "User Information")]
#[get("/@me/profile")]
pub async fn req(user: User) -> Result<Json<UserProfile>> {
    Ok(Json(user.profile.unwrap_or_default()))
}
