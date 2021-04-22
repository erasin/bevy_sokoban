pub struct AssetPaths {
    pub font_ui: &'static str,
    pub audio_wall: &'static str,
    pub texture_sheet: &'static str,
    pub texture_player: &'static str,
    pub texture_box_blue: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    font_ui: "fonts/KenneyFuture.ttf",
    audio_wall: "sounds/wall.wav",
    texture_sheet: "textures/sheet.png",
    texture_player: "textures/player_sheet.png",
    texture_box_blue: "textures/box_blue_sheet.png",
};
