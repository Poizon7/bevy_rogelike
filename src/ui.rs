use bevy::prelude::*;

use crate::Selected;

#[derive(Component)]
pub struct TaskBar;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).insert(TaskBar)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "M",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.0, 0.0),
                        },
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(60.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "B",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.0, 0.0),
                        },
                    ));
               });
        });
}

pub fn taskbar_visibility_system(taskbars: Query<Entity, With<TaskBar>>, selected: Query<Entity, With<Selected>>, mut commands: Commands) {
    if matches!(selected.iter().next(), None) {
        for entity in taskbars.iter() {
            commands.entity(entity).remove::<Visibility>();
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    else {
        for entity in taskbars.iter() {
            commands.entity(entity).remove::<Visibility>();
            commands.entity(entity).insert(Visibility::Visible);
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
    selected: Query<Entity, With<Selected>>,
    mut commands: Commands,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                if text.sections[0].value == "M" {
                    for entity in selected.iter() {
                        commands.entity(entity).remove::<Selected>();
                        commands.entity(entity).insert(Selected::Movable);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

