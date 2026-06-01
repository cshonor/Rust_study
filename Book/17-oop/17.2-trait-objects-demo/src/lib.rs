//! 极简 GUI 示例：用 `draw` 统一行为；`Screen` 持有 `Vec<Box<dyn Draw>>`。

/// 可在屏幕上绘制的组件。
pub trait Draw {
    fn draw(&self);
}

/// 屏幕：持有若干组件并统一调用 `draw`（trait 对象，运行时多态）。
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// 库提供的按钮组件（示例，无真实 GUI 渲染）。
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button {}x{} {:?}", self.width, self.height, self.label);
    }
}
