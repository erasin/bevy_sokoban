use crate::{
    加载模块::字体素材, 地图模块::地图数据, 数据模块::*, 状态模块::全局状态
};
use bevy::prelude::*;

#[derive(Default)]
pub struct 界面组件;

impl Plugin for 界面组件 {
    fn build(&self, app: &mut App) {
        // app.init_resource::<ButtonMaterials>()
        app.add_system_set(SystemSet::on_enter(全局状态::游戏中).with_system(初始化处理))
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中)
                    // .with_system(按钮处理)
                    .with_system(文本变动),
            );
    }
}

#[derive(Component)]
pub struct 界面层;

fn 初始化处理(mut 指令: Commands, 数据: Res<全局数据>, 字体: Res<字体素材>) {
    指令
        .spawn_bundle(NodeBundle {
            style: Style {
                //  100%
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween, // 对齐方式
                flex_direction: FlexDirection::Column,         // 主轴 行结构，默认列
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }) // 总节点
        .insert(界面层)
        .with_children(|节点| {
            // 行1
            节点
                .spawn()
                .insert_bundle(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        // align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::hex("81C784").unwrap().into(),
                    ..Default::default()
                })
                .with_children(|行| {
                    // 列1
                    // 行.spawn()
                    //     .insert_bundle(NodeBundle {
                    //         style: Style {
                    //             align_items: AlignItems::Center,
                    //             justify_content: JustifyContent::Center,
                    //             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    //             ..Default::default()
                    //         },
                    //         material: 材质.add(Color::hex("7CB342").unwrap().into()),
                    //         ..Default::default()
                    //     })
                    //     .with_children(|列| {
                    //         列.spawn()
                    //             .insert_bundle(ButtonBundle {
                    //                 style: Style {
                    //                     size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    //                     // center button
                    //                     margin: Rect::all(Val::Auto),
                    //                     // horizontally center child text
                    //                     justify_content: JustifyContent::Center,
                    //                     // vertically center child text
                    //                     align_items: AlignItems::Center,
                    //                     ..Default::default()
                    //                 },
                    //                 material: 按钮材质.normal.as_weak(),
                    //                 ..Default::default()
                    //             })
                    //             .insert(UI按钮下一关)
                    //             .with_children(|parent| {
                    //                 parent.spawn().insert_bundle(TextBundle {
                    //                     text: Text::with_section(
                    //                         "下一关".to_string(),
                    //                         TextStyle {
                    //                             font_size: 20.0,
                    //                             font: 字体.font_ui.as_weak(),
                    //                             color: Color::rgb(0.8, 0.8, 0.8),
                    //                         },
                    //                         TextAlignment::default(),
                    //                     ),
                    //                     ..Default::default()
                    //                 });
                    //             });
                    //     });

                    // 列2
                    行.spawn()
                        .insert_bundle(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            color: Color::hex("F57C00").unwrap().into(),
                            ..Default::default()
                        })
                        .with_children(|列| {
                            列.spawn()
                                .insert_bundle(TextBundle {
                                    text: Text::from_section(
                                        format!("计步:{}", 数据.计步数),
                                        TextStyle {
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                            font: 字体.font_ui.as_weak(),
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(UI计步器);
                        });

                    // 列3
                    行.spawn()
                        .insert_bundle(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            color: Color::hex("0288D1").unwrap().into(),
                            ..Default::default()
                        })
                        .with_children(|列| {
                            列.spawn()
                                .insert_bundle(TextBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    text: Text::from_section(
                                        format!("p:{}", 数据.踩点),
                                        TextStyle {
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                            font: 字体.font_ui.as_weak(),
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(UI目标计数);
                        });
                });
        });
}

fn 文本变动(
    数据: Res<全局数据>,
    地图: Res<地图数据>,
    // mut step_query: Query<(&mut Text, &UIStep)>,
    // mut spot_query: Query<(&mut Text, &UISpot)>,
    mut query: ParamSet<(
        Query<&mut Text, With<UI计步器>>,
        Query<&mut Text, With<UI目标计数>>,
    )>,
) {
    for mut t in query.p0().iter_mut() {
        t.sections[0].value = format!("step:{}", 数据.计步数);
    }
    for mut t in query.p1().iter_mut() {
        t.sections[0].value = format!("P:{}/{}", 数据.踩点, 地图.目标数量);
    }
}

// pub struct ButtonMaterials {
//     pub normal: Handle<ColorMaterial>,
//     pub hovered: Handle<ColorMaterial>,
//     pub pressed: Handle<ColorMaterial>,
// }

// impl FromWorld for ButtonMaterials {
//     fn from_world(world: &mut World) -> Self {
//         let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
//         ButtonMaterials {
//             normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
//             hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
//             pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
//         }
//     }
// }

#[derive(Component)]
pub struct UI计步器;
#[derive(Component)]
pub struct UI目标计数;
// pub struct UI按钮下一关;

// fn 按钮处理(
//     button_materials: Res<ButtonMaterials>,
//     mut interaction_query: Query<
//         (
//             &Button,
//             &Interaction,
//             &mut Handle<ColorMaterial>,
//             &Children,
//             &UI按钮下一关,
//         ),
//         Changed<Interaction>,
//     >,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (_button, interaction, mut material, children, _) in interaction_query.iter_mut() {
//         let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Clicked => {
//                 text.sections[0].value = "Press".to_string();
//                 *material = button_materials.pressed.as_weak();
//                 println!("Press ok");
//                 // TODO load map
//             }
//             Interaction::Hovered => {
//                 text.sections[0].value = "Hover".to_string();
//                 *material = button_materials.hovered.as_weak();
//             }
//             Interaction::None => {
//                 text.sections[0].value = "Button".to_string();
//                 *material = button_materials.normal.as_weak();
//             }
//         }
//     }
// }
