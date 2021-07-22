use crate::{加载模块::FontAssets, 数据模块::*, 状态模块::全局状态};
use bevy::prelude::*;

#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(全局状态::游戏中).with_system(初始化处理.system()))
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中)
                    .with_system(按钮处理.system())
                    .with_system(文本变动.system()),
            );
    }
}

fn 初始化处理(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    data: Res<全局数据>,
    font: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn().insert_bundle(UiCameraBundle::default());

    commands
        .spawn()
        .insert_bundle(NodeBundle {
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
                .spawn()
                .insert_bundle(NodeBundle {
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
                        .spawn()
                        .insert_bundle(NodeBundle {
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
                                .spawn()
                                .insert_bundle(ButtonBundle {
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
                                    material: button_materials.normal.as_weak(),
                                    ..Default::default()
                                })
                                .insert(UIBtnNext)
                                .with_children(|parent| {
                                    parent.spawn().insert_bundle(TextBundle {
                                        text: Text::with_section(
                                            "btn".to_string(),
                                            TextStyle {
                                                font_size: 20.0,
                                                font: font.font_ui.as_weak(),
                                                color: Color::rgb(0.8, 0.8, 0.8),
                                            },
                                            TextAlignment::default(),
                                        ),
                                        ..Default::default()
                                    });
                                });
                        });

                    // 列2
                    parent
                        .spawn()
                        .insert_bundle(NodeBundle {
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
                                .spawn()
                                .insert_bundle(TextBundle {
                                    text: Text::with_section(
                                        format!("step:{}", data.计步数),
                                        TextStyle {
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                            font: font.font_ui.as_weak(),
                                        },
                                        TextAlignment::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(UIStep);
                        });

                    // 列3
                    parent
                        .spawn()
                        .insert_bundle(NodeBundle {
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
                                .spawn()
                                .insert_bundle(TextBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        format!("p:{}", data.踩点),
                                        TextStyle {
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                            font: font.font_ui.as_weak(),
                                        },
                                        TextAlignment::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(UISpot);
                        });
                });
        });
}

fn 文本变动(
    data: Res<全局数据>,
    // mut step_query: Query<(&mut Text, &UIStep)>,
    // mut spot_query: Query<(&mut Text, &UISpot)>,
    mut query: QuerySet<(
        Query<&mut Text, With<UIStep>>,
        Query<&mut Text, With<UISpot>>,
    )>,
) {
    for mut t in query.q0_mut().iter_mut() {
        t.sections[0].value = format!("step:{}", data.计步数);
    }
    for mut t in query.q1_mut().iter_mut() {
        t.sections[0].value = format!("P:{}", data.踩点);
    }
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
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

fn 按钮处理(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (
            &Button,
            &Interaction,
            &mut Handle<ColorMaterial>,
            &Children,
            &UIBtnNext,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children, _) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.as_weak();
                println!("Press ok");
                // load map
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *material = button_materials.hovered.as_weak();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *material = button_materials.normal.as_weak();
            }
        }
    }
}
