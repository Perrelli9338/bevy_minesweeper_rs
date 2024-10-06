use crate::{
    resources::settings::GameSettings,
    scenes::{cleanup, ButtonColors, ChangeState, MenuButtonAction, MenuStates, H1},
    widgets::{button::UiButtonWidgetExt, text::UiTextWidgetExt, settings::UiSettingsWidgetExt, tab_container::TabContainerExt},
    AppState,
};
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component)]
pub enum SettingsMenuButtonAction {
    DecrementBombCount,
    IncrementBombCount,
    IncrementWidthBoard,
    DecrementWidthBoard,
    IncrementHeightBoard,
    DecrementHeightBoard,
    SafeStartOn,
    SafeStartOff,
    TurnFlagOn,
    TurnFlagOff,
    IncreaseStartTimer,
    DecreaseStartTimer,
    IncreaseTouchTimer,
    DecreaseTouchTimer,
}

#[derive(Component)]
pub struct SettingsValues;

#[derive(Component)]
pub struct MenuSettings;

pub struct SettingsMenu;

impl Plugin for SettingsMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Settings), Self::create)
            .add_systems(
                Update,
                (
                    Self::settings_button_functions,
                    Self::settings_button_colors,
                )
                    .run_if(in_state(MenuStates::Settings)),
            )
            .add_systems(OnExit(MenuStates::Settings), cleanup::<MenuSettings>);
    }
}

impl SettingsMenu {
    fn create(mut commands: Commands, options: Option<Res<GameSettings>>) {
        let config = match options {
            None => GameSettings::default(),
            Some(c) => c.clone(),
        };

        commands
            .ui_builder(UiRoot)
            .container(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(90.0),
                    margin: UiRect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.),
                    ..default()
                },
                ..default()
            },
                       |children| {
                           children.text("Settings", None).insert(H1);
                           children.container(
                               NodeBundle {
                                   style: Style {
                                       display: Display::Flex,
                                       flex_direction: FlexDirection::Column,
                                       align_items: AlignItems::FlexEnd,
                                       width: Val::Percent(80.0),
                                       height: Val::Percent(100.0),
                                       ..default()
                                   },
                                   ..default()
                               },
                               |children| {
                                   children.row(|row| {
                                       row.docking_zone_split(
                                           SizedZoneConfig {
                                               size: 25.,
                                               ..default()
                                           },
                                           |right_side| {
                                               right_side.docking_zone(
                                                   SizedZoneConfig {
                                                       size: 25.,
                                                       ..default()
                                                   },
                                                   true,
                                                   |bar| {
                                                       bar.add_tab_container("Grid", |children| {
                                                           children.settings(SettingsMenuButtonAction::DecrementWidthBoard, SettingsMenuButtonAction::IncrementWidthBoard, "Width",  &config.map_size.0.to_string());
                                                           children.settings(SettingsMenuButtonAction::DecrementHeightBoard, SettingsMenuButtonAction::IncrementHeightBoard, "Height",  &config.map_size.1.to_string());
                                                           children.settings(SettingsMenuButtonAction::DecrementBombCount, SettingsMenuButtonAction::IncrementBombCount, "Bombs",  &config.bomb_count.to_string());
                                                       }).style_inplace(|style| {
                                                           style.background_color(Color::linear_rgb(0.2, 0.2, 0.2));
                                                       }).style_unchecked();
                                                       bar.add_tab_container("Game".into(), |children| {
                                                               children.settings(SettingsMenuButtonAction::SafeStartOff, SettingsMenuButtonAction::SafeStartOn, "Safe start",  & match config.easy_mode {
                                                               true => "On",
                                                               false => "Off",
                                                           }
                                                               .to_string());
                                                               children.settings(SettingsMenuButtonAction::TurnFlagOff, SettingsMenuButtonAction::TurnFlagOn, "Flag mode",  &match config.flag_mode {
                                                               true => "On",
                                                               false => "Off",
                                                           }
                                                               .to_string());
                                                       });
                                                       bar.add_tab_container("Accessibility".into(), |children| {
                                                               children.settings(SettingsMenuButtonAction::DecreaseStartTimer, SettingsMenuButtonAction::IncreaseStartTimer, "Start delay",  &format!("{:.01}s", config.timer_start));
                                                               children.settings(SettingsMenuButtonAction::DecreaseTouchTimer, SettingsMenuButtonAction::IncreaseTouchTimer, "Touch delay",  &format!("{:.2}s", config.timer_touch));
                                                   });
                                                   },
                                               );
                                           },
                                       );
                                   }).style().height(Val::Percent(100.));
                               });
                           children.button_main_menu("Close", MenuButtonAction::BackToMainMenu);
                       }).insert(MenuSettings);
    }

    fn settings_button_functions(
        mut commands: Commands,
        mut query: Query<&mut Text, With<SettingsValues>>,
        mut interaction_query: Query<
            (&Interaction, &SettingsMenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut config: ResMut<GameSettings>,
    ) {
        for (interaction, button_action) in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                match button_action {
                    SettingsMenuButtonAction::DecrementBombCount => {
                        if config.bomb_count > 1 {
                            config.bomb_count -= 1;
                        }
                    }
                    SettingsMenuButtonAction::IncrementBombCount => {
                        if config.bomb_count < (config.map_size.0 * config.map_size.1) - 1 {
                            config.bomb_count += 1;
                        }
                    }
                    SettingsMenuButtonAction::IncrementWidthBoard => {
                        if config.map_size.0 <= 200 {
                            config.map_size.0 += 1;
                        }
                    }
                    SettingsMenuButtonAction::DecrementWidthBoard => {
                        if config.map_size.0 > 1
                            && ((config.map_size.0 - 1) * config.map_size.1) > config.bomb_count
                        {
                            config.map_size.0 -= 1;
                        }
                    }
                    SettingsMenuButtonAction::DecrementHeightBoard => {
                        if config.map_size.1 > 1
                            && (config.map_size.0 * (config.map_size.1 - 1)) > config.bomb_count
                        {
                            config.map_size.1 -= 1;
                        }
                    }
                    SettingsMenuButtonAction::IncrementHeightBoard => {
                        if config.map_size.1 <= 200 {
                            config.map_size.1 += 1;
                        }
                    }
                    SettingsMenuButtonAction::SafeStartOn => {
                        config.easy_mode = true;
                    }
                    SettingsMenuButtonAction::SafeStartOff => {
                        config.easy_mode = false;
                    }
                    SettingsMenuButtonAction::DecreaseStartTimer => {
                        if config.timer_start > 0.0 {
                            config.timer_start = (config.timer_start * 10.0 - 1.0) / 10.0;
                        }
                    }
                    SettingsMenuButtonAction::IncreaseStartTimer => {
                        if config.timer_start < 3.0 {
                            config.timer_start += 0.1;
                        }
                    }
                    SettingsMenuButtonAction::TurnFlagOn => {
                        config.flag_mode = true;
                    }
                    SettingsMenuButtonAction::TurnFlagOff => {
                        config.flag_mode = false;
                    }
                    SettingsMenuButtonAction::DecreaseTouchTimer => {
                        if config.timer_touch > 0.01 {
                            config.timer_touch = (config.timer_touch * 100.0 - 1.0) / 100.0;
                        }
                    }
                    SettingsMenuButtonAction::IncreaseTouchTimer => {
                        if config.timer_touch < 3.0 {
                            config.timer_touch += 0.01;
                        }
                    }
                }
                if (config.bomb_count == (config.map_size.0 * config.map_size.1) - 1
                    || config.bomb_count == 1)
                    && config.easy_mode
                {
                    config.flag_mode = true
                }
                let mut settings_values = vec![
                    format!("{:.2}s", config.timer_touch),
                    format!("{:.01}s", config.timer_start),
                    match config.flag_mode {
                        true => "On",
                        false => "Off",
                    }
                        .to_string(),
                    match config.easy_mode {
                        true => "On",
                        false => "Off",
                    }
                        .to_string(),
                    config.bomb_count.to_string(),
                    config.map_size.1.to_string(),
                    config.map_size.0.to_string(),
                ];
                for mut b in query.iter_mut() {
                    b.sections[0].value = settings_values.pop().unwrap();
                }
                commands.insert_resource(GameSettings {
                    map_size: config.map_size,
                    bomb_count: config.bomb_count,
                    position: config.clone().position,
                    tile_size: config.clone().tile_size,
                    tile_padding: config.tile_padding,
                    easy_mode: config.easy_mode,
                    timer_start: config.timer_start,
                    timer_touch: config.timer_touch,
                    flag_mode: config.flag_mode,
                })
            }
        }
    }

    fn settings_button_colors(
        mut next_state: ResMut<NextState<AppState>>,
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &ButtonColors,
                &SettingsMenuButtonAction,
                Option<&ChangeState>,
            ),
            With<Button>,
        >,
        mut config: ResMut<GameSettings>,
    ) {
        for (interaction, mut color, button_colors, button_action, change_state) in
            &mut interaction_query
        {
            match *interaction {
                Interaction::None => {
                    *color = button_colors.normal.into();
                }
                Interaction::Pressed => {
                    *color = button_colors.pressed.into();
                    if let Some(state) = change_state {
                        next_state.set(state.0.clone());
                    }
                }
                Interaction::Hovered => {
                    *color = button_colors.hovered.into();
                }
            }
            match button_action {
                SettingsMenuButtonAction::DecrementBombCount => {
                    if !(config.bomb_count > 1) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::IncrementBombCount => {
                    if !(config.bomb_count < (config.map_size.0 * config.map_size.1) - 1) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::IncrementWidthBoard => {
                    if config.map_size.0 <= 200 {
                    } else {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::DecrementWidthBoard => {
                    if !(config.map_size.0 > 1
                        && ((config.map_size.0 - 1) * config.map_size.1) > config.bomb_count)
                    {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::DecrementHeightBoard => {
                    if !(config.map_size.1 > 1
                        && (config.map_size.0 * (config.map_size.1 - 1)) > config.bomb_count)
                    {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::IncrementHeightBoard => {
                    if !(config.map_size.1 <= 200) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::SafeStartOn => {
                    if !config.easy_mode {
                    } else {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::SafeStartOff => {
                    if !config.easy_mode {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::DecreaseStartTimer => {
                    if !(config.timer_start > 0.) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::IncreaseStartTimer => {
                    if !(config.timer_start < 3.0) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::DecreaseTouchTimer => {
                    if !(config.timer_touch > 0.01) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::IncreaseTouchTimer => {
                    if !(config.timer_touch < 3.0) {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::TurnFlagOn => {
                    if config.flag_mode {
                        *color = button_colors.disabled.into();
                    }
                }
                SettingsMenuButtonAction::TurnFlagOff => {
                    if !config.flag_mode
                        || (config.bomb_count == (config.map_size.0 * config.map_size.1) - 1
                            || config.bomb_count == 1)
                            && config.easy_mode
                    {
                        *color = button_colors.disabled.into();
                    }
                }
            }
        }
    }
}
