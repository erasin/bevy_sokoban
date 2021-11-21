use crate::全局状态;
use crate::加载模块::字体素材;
use crate::地图模块::地图加载事件;
use crate::界面模块::界面层;
use crate::组件模块::坐标;
use bevy::ecs::component::Component;
use bevy::prelude::*;

/// 菜单组件
pub struct 主菜单组件;

impl Plugin for 主菜单组件 {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(全局状态::主菜单)
                    .with_system(清理内容::<坐标>.system())
                    .with_system(清理内容::<界面层>.system())
                    .with_system(初始化处理.system()),
            )
            .add_system_set(
                SystemSet::on_update(全局状态::主菜单)
                    .with_system(点击开始按钮处理.system())
                    .with_system(键盘处理.system()),
            )
            .add_system_set(
                SystemSet::on_exit(全局状态::主菜单).with_system(清理内容::<菜单层>.system()),
            );
    }
}

/// 按钮事件
struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    // pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            // pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

pub fn 清理内容<T: Component>(mut 指令: Commands, 所有: Query<Entity, With<T>>) {
    所有.for_each(|e| {
        指令.entity(e).despawn_recursive();
    });
}

pub struct 菜单层;

/// 按钮
struct 开始按钮;

fn 初始化处理(
    // 世界: &mut World,
    mut 指令: Commands,
    字体资源: Res<字体素材>,
    按钮素材: Res<ButtonMaterials>,
) {
    指令.spawn_bundle(UiCameraBundle::default());
    指令
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: 按钮素材.normal.clone(),
            ..Default::default()
        })
        .insert_bundle((菜单层, 开始按钮))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: 字体资源.font_ui.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn 点击开始按钮处理(
    按钮材质: Res<ButtonMaterials>,
    mut 当前状态: ResMut<State<全局状态>>,
    mut 交互队列: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (With<开始按钮>, Changed<Interaction>),
    >,
    mut 地图加载事件发送器: EventWriter<地图加载事件>,
) {
    for (交互类型, mut 材质) in 交互队列.iter_mut() {
        match *交互类型 {
            Interaction::Clicked => {
                地图加载事件发送器.send(地图加载事件(1));
                当前状态.set(全局状态::游戏中).unwrap();
            }
            Interaction::Hovered => {
                *材质 = 按钮材质.hovered.clone();
            }
            Interaction::None => {
                *材质 = 按钮材质.normal.clone();
            }
        }
    }
}

fn 键盘处理(
    mut 按键键值: ResMut<Input<KeyCode>>,
    mut 当前状态: ResMut<State<全局状态>>,
    mut 地图加载事件发送器: EventWriter<地图加载事件>,
) {
    if 按键键值.just_released(KeyCode::Return) {
        地图加载事件发送器.send(地图加载事件(1));
        当前状态.set(全局状态::游戏中).unwrap();
        按键键值.reset(KeyCode::Grave);
    }
}
