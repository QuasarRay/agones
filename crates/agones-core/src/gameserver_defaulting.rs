//! Source defaulting policy and its context-owned dependencies.
use cgp::prelude::*;

use crate::{
    AgonesBuildVersion, AgonesBuildVersionGetterComponent, AgonesVersionAnnotationGetterComponent,
    CanGetAgonesVersionAnnotation, EncodeAgonesVersionAnnotation, FeatureFlags, FeatureGates,
    GameServer,
};

#[cgp_getter]
pub trait HasGameServerFeatureFlags {
    fn game_server_feature_flags(&self) -> &FeatureFlags;
}

#[cgp_component(GameServerDefaulter)]
pub trait CanApplyGameServerDefaults {
    fn apply_game_server_defaults(&self, game_server: &mut GameServer);
}

#[cgp_new_provider]
impl<Context> GameServerDefaulter<Context> for ApplyAgonesGameServerDefaults
where
    Context: CanGetAgonesVersionAnnotation + HasGameServerFeatureFlags,
{
    fn apply_game_server_defaults(context: &Context, game_server: &mut GameServer) {
        game_server.apply_defaults_with(
            &context.agones_version_annotation(),
            *context.game_server_feature_flags(),
        );
    }
}

#[derive(Clone, Debug, Default, HasField)]
pub struct GameServerDefaultingContext {
    pub agones_build_version: AgonesBuildVersion,
    pub game_server_feature_flags: FeatureFlags,
}

impl GameServerDefaultingContext {
    pub fn from_feature_gates(gates: &FeatureGates) -> Self {
        Self {
            game_server_feature_flags: FeatureFlags {
                player_tracking: gates.enabled("PlayerTracking"),
            },
            ..Self::default()
        }
    }
}

delegate_and_check_components! {
    GameServerDefaultingContext {
        AgonesBuildVersionGetterComponent: UseField<Symbol!("agones_build_version")>,
        AgonesVersionAnnotationGetterComponent: EncodeAgonesVersionAnnotation,
        GameServerFeatureFlagsGetterComponent: UseField<Symbol!("game_server_feature_flags")>,
        GameServerDefaulterComponent: ApplyAgonesGameServerDefaults,
        crate::GameServerFeatureValidatorComponent: crate::ValidateAgonesGameServerFeatures,
        crate::ResourceQuantityValidatorComponent: crate::ValidateAgonesResourceQuantities,
    }
}
