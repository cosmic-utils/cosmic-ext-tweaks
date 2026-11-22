use cosmic::{
    Application,
    cosmic_config::{
        self, Config, ConfigGet, ConfigSet, CosmicConfigEntry,
        cosmic_config_derive::CosmicConfigEntry,
    },
    theme,
};
use cosmic_panel_config::CosmicPanelConfig;
use serde::{Deserialize, Serialize};

use crate::app::App;

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize, CosmicConfigEntry)]
pub struct TweaksConfig {
    pub app_theme: AppTheme,
}

impl TweaksConfig {
    pub fn config() -> Config {
        match Config::new(App::APP_ID, CONFIG_VERSION) {
            Ok(config) => config,
            Err(err) => panic!("Failed to fetch config for application: {err}"),
        }
    }

    pub fn new() -> TweaksConfig {
        TweaksConfig::get_entry(&Self::config()).unwrap_or_else(|(errs, config)| {
            log::info!("errors loading config: {:?}", errs);
            config
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
    #[default]
    System,
}

impl AppTheme {
    pub fn theme(&self) -> theme::Theme {
        match self {
            Self::Dark => {
                let mut t = theme::system_dark();
                t.theme_type.prefer_dark(Some(true));
                t
            }
            Self::Light => {
                let mut t = theme::system_light();
                t.theme_type.prefer_dark(Some(false));
                t
            }
            Self::System => theme::system_preference(),
        }
    }
}

pub fn load_panel_config(helper: &Config, name: &str) -> Option<CosmicPanelConfig> {
    let mut config = CosmicPanelConfig::get_entry(helper).ok()?;
    if config.name != name {
        return None;
    }

    for (legacy_key, override_key, value) in [
        ("padding", "padding_override", &mut config.padding_override),
        (
            "border_radius",
            "border_radius_override",
            &mut config.border_radius_override,
        ),
    ] {
        // An explicitly cleared override is stored as None.
        if matches!(
            helper.get_local::<Option<u32>>(override_key),
            Err(cosmic_config::Error::NotFound)
        ) && let Ok(legacy_value) = helper.get_local::<u32>(legacy_key)
        {
            let migrated = if legacy_key == "padding" {
                padding_override(legacy_value)
            } else {
                Some(legacy_value)
            };
            match helper.set(override_key, migrated) {
                Ok(()) => *value = migrated,
                Err(err) => log::error!("Error migrating {name} {legacy_key}: {err}"),
            }
        }
    }

    Some(config)
}

pub fn padding_override(padding: u32) -> Option<u32> {
    (padding != 0).then_some(padding)
}

pub fn panel_border_radius(config: &CosmicPanelConfig) -> u32 {
    config.border_radius_override.unwrap_or_else(|| {
        if !config.anchor_gap && config.expand_to_edges {
            return 0;
        }
        let radius = theme::system_preference().cosmic().corner_radii.radius_xl[0] as u32;
        if config.anchor_gap {
            radius
        } else {
            radius.min(12)
        }
    })
}
