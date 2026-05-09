use warp_core::features::FeatureFlag;
use warpui::{AppContext, SingletonEntity as _};

use super::AuthStateProvider;

pub fn local_loginless_mode(ctx: &AppContext) -> bool {
    FeatureFlag::LocalLoginlessMode.is_enabled()
        || AuthStateProvider::as_ref(ctx).get().is_local_loginless()
}

pub fn official_warp_account_available(ctx: &AppContext) -> bool {
    !local_loginless_mode(ctx) && AuthStateProvider::as_ref(ctx).get().has_official_account()
}

pub fn official_warp_cloud_enabled(ctx: &AppContext) -> bool {
    official_warp_account_available(ctx)
}
