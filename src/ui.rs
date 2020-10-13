use crate::data::*;
use crate::resources::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(setup_system.system())
            .add_system(button_system.system())
            .add_system(text_system.system());
    }
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resource: Res<ResourceData>,
    data: Res<GameData>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                //  100%
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween, // 对齐方式
                flex_direction: FlexDirection::Column,         // 主轴 行结构，默认列
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        }) // 总节点
        .with_children(|parent| {
            // 行1
            parent
                .spawn(NodeComponents {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        // align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::hex("81C784").unwrap().into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // 列1
                    parent
                        .spawn(NodeComponents {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::hex("7CB342").unwrap().into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonComponents {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // center button
                                        margin: Rect::all(Val::Auto),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    material: button_materials.normal,
                                    ..Default::default()
                                })
                                .with(UIBtnNext)
                                .with_children(|parent| {
                                    parent.spawn(TextComponents {
                                        text: Text {
                                            value: "Button".to_string(),
                                            font: resource.ui_font,
                                            style: TextStyle {
                                                font_size: 20.0,
                                                color: Color::rgb(0.8, 0.8, 0.8),
                                            },
                                        },
                                        ..Default::default()
                                    });
                                });
                        })
                        .spawn(NodeComponents {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::hex("F57C00").unwrap().into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!("Step:{}", data.step),
                                        font: resource.ui_font,
                                        style: TextStyle {
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(UIStep);
                        })
                        .spawn(NodeComponents {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::hex("0288D1").unwrap().into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: format!("Spot:{}", data.spot),
                                        font: resource.ui_font,
                                        style: TextStyle {
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(UISpot);
                        });
                });
        });
}

fn text_system(
    data: Res<GameData>,
    mut step_query: Query<(&mut Text, &UIStep)>,
    mut spot_query: Query<(&mut Text, &UISpot)>,
) {
    for (mut t, _) in &mut step_query.iter() {
        t.value = format!("Step:{}", data.step);
    }
    for (mut t, _) in &mut spot_query.iter() {
        t.value = format!("Spot:{}", data.spot);
    }
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

pub struct UIStep;
pub struct UISpot;
pub struct UIBtnNext;
pub struct UIBtnPrev;

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
        &UIBtnNext,
    )>,
    text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children, _) in &mut interaction_query.iter() {
        let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = button_materials.pressed;
                println!("Press ok");
                // load map
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = button_materials.hovered;
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = button_materials.normal;
            }
        }
    }
}
