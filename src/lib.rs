use bevy::{app::PluginGroupBuilder, prelude::*};

macro_rules! features {
    ($(($feature:literal, $mod:ident, $plugin:ident)),+) => {
        $(
            #[cfg(feature = $feature)]
            pub mod $mod;
        )+

        pub struct TinaePlugins;

        impl PluginGroup for TinaePlugins {
            fn build(self) -> PluginGroupBuilder {
                let mut group = PluginGroupBuilder::start::<Self>();

                $(
                    #[cfg(feature = $feature)]
                    {
                        group = group.add(crate::$mod::$plugin);
                    }
                )+

                group
            }
        }

        pub mod prelude {
            pub use super::{TinaePlugins, Persistent};
            $(
                #[cfg(feature = $feature)]
                pub use super::$mod::prelude::*;
            )+
        }
    };
}

features!(
    ("tinae_asset_struct", asset_struct, AssetStructPlugin),
    ("tinae_cursor", cursor, CursorPlugin),
    ("tinae_fixed_timestep", fixed_timestep, FixedTimestepPlugin),
    ("tinae_flow", flow, FlowPlugin),
    ("tinae_force_ratio", force_ratio, ForceRatioPlugin),
    ("tinae_geometry", geometry, GeometryPlugin),
    ("tinae_scenes", scenes, ScenesPlugin),
    ("tinae_screen_fade", screen_fade, ScreenFadePlugin),
    ("tinae_spine", spine, SpinePlugin),
    ("tinae_sub_assets", sub_assets, SubAssetsPlugin),
    ("tinae_time_to_live", time_to_live, TimeToLivePlugin),
    ("tinae_transform2", transform2, Transform2Plugin)
);

#[derive(Component)]
pub struct Persistent;
