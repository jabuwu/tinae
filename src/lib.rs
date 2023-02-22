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
            pub use super::TinaePlugins;
            $(
                pub use super::$mod::prelude::*;
            )+
        }
    };
}

features!(
    ("tinae_cursor", cursor, CursorPlugin),
    ("tinae_screen_fade", screen_fade, ScreenFadePlugin),
    ("tinae_transform2", transform2, Transform2Plugin)
);
