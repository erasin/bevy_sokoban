# sokoban 推箱子

* 参考文件 [Rust sokoban](https://sokoban.iolivia.me/)

![game](assets/game.png)

## ecs 

**实体**

| 实体     | 名称     | 组件                 |
| -------- | -------- | -------------------- |
| Player   | 用户     | 位置,可渲染,可移动   |
| Wall     | 墙       | 位置,可渲染,不可移动 |
| Floor    | 地板     | 位置,可渲染          |
| Box      | 箱子     | 位置,可渲染,可移动   |
| Box spot | 箱子目标 | 位置,可渲染          |

**组件**

| 组件       | 名称     | 定义 |
| ---------- | -------- | ---- |
| Position   | 位置     |
| Renderable | 可渲染   |
| Movable    | 可移动   |
| Immovable  | 不可移动 |

**资源**

**事件**

**系统**

* 地图加载
* 用户移动
  * 推动箱子
* box spot 终点检测
* 坐标转换
* 组件渲染
* 计分器
* 事件监听


