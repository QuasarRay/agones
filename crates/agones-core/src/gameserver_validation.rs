//! Source feature-gate validation, selected through the shared CGP configuration.
use cgp::prelude::*;

use crate::{GameServerSpec, HasGameServerFeatureFlags, ValidationError, ValidationErrorKind};

#[cgp_component(GameServerFeatureValidator)]
pub trait CanValidateGameServerFeatures {
    fn validate_game_server_features(
        &self,
        spec: &GameServerSpec,
        path: &str,
    ) -> Vec<ValidationError>;
}

#[cgp_new_provider]
impl<Context> GameServerFeatureValidator<Context> for ValidateAgonesGameServerFeatures
where
    Context: HasGameServerFeatureFlags,
{
    fn validate_game_server_features(
        context: &Context,
        spec: &GameServerSpec,
        path: &str,
    ) -> Vec<ValidationError> {
        if !context.game_server_feature_flags().player_tracking && spec.players.is_some() {
            vec![ValidationError {
                kind: ValidationErrorKind::Forbidden,
                path: if path.is_empty() {
                    "players".to_owned()
                } else {
                    format!("{path}.players")
                },
                bad_value: None,
                detail: "Value cannot be set unless feature flag PlayerTracking is enabled"
                    .to_owned(),
            }]
        } else {
            Vec::new()
        }
    }
}
