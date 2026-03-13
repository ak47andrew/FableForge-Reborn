use raylib::prelude::Color;
use crate::widgets::ButtonMode;

pub struct ButtonStyle {
    pub disabled_color: Color,
    pub normal_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
}

impl ButtonStyle {
    pub fn new(disabled_color: Color, normal_color: Color, hovered_color: Color, pressed_color: Color) -> ButtonStyle {
        ButtonStyle {
            disabled_color,
            normal_color,
            hovered_color,
            pressed_color,
        }
    }
    pub fn get_button_color(&self, mode: &ButtonMode) -> Color {
        match mode {
            ButtonMode::Disabled => {self.disabled_color.clone()},
            ButtonMode::Normal => {self.normal_color.clone()},
            ButtonMode::Hovered => {self.hovered_color.clone()},
            ButtonMode::Pressed => {self.pressed_color.clone()},
        }
    }
}

impl ButtonStyle {
    pub const STYLE_BLUE: Self = Self {
        disabled_color: Color::new(150, 150, 170, 255),  // Disabled
        normal_color: Color::new(65, 105, 225, 255),   // Normal (Royal Blue)
        hovered_color: Color::new(100, 149, 237, 255),  // Hovered (Light Sky Blue)
        pressed_color: Color::new(25, 25, 112, 255)     // Pressed (Midnight Blue)
    };
    pub const STYLE_PRIMARY: Self = Self::STYLE_BLUE;

    // Success/Confirm (Green)
    pub const STYLE_GREEN: Self = Self {
        disabled_color: Color::new(170,200,170,255),
        normal_color: Color::new(50,205,50,255),    // LimeGreen
        hovered_color: Color::new(144,238,144,255),  // LightGreen
        pressed_color: Color::new(0,100,0,255)       // DarkGreen
    };
    pub const STYLE_SUCCESS: Self = Self::STYLE_GREEN;

    // Warning (Orange/Yellow)
    pub const STYLE_ORANGE: Self = Self {
        disabled_color: Color::new(180, 170, 150, 255),
        normal_color: Color::new(255, 140, 0, 255),    // DarkOrange
        hovered_color: Color::new(255, 215, 0, 255),    // Gold
        pressed_color: Color::new(205, 102, 0, 255)     // DarkOrange (darker)
    };
    pub const STYLE_WARNING: Self = Self::STYLE_ORANGE;

    // Minimalist Light
    pub const STYLE_MINIMAL_LIGHT: Self = Self {
        disabled_color: Color::new(230,230,230,180),
        normal_color: Color::new(240,240,240,255),
        hovered_color: Color::new(220,220,220,255),
        pressed_color: Color::new(200,200,200,255)
    };

    // Dark Theme
    pub const STYLE_DARK: Self = Self {
        disabled_color: Color::new(40,40,40,200),
        normal_color: Color::new(50,50,60,255),
        hovered_color: Color::new(70,70,90,255),
        pressed_color: Color::new(30,30, 40, 255)
    };

    // Purple Accent
    pub const STYLE_PURPLE: Self = Self {
        disabled_color: Color::new(180,160,190,255),
        normal_color: Color::new(147,112,219,255),  // MediumPurple
        hovered_color: Color::new(186,85,211,255),   // MediumOrchid
        pressed_color: Color::new(75,0,130,255)      // Indigo
    };

    // Glass/Transparent Effect
    pub const STYLE_GLASS: Self = Self {
        disabled_color: Color::new(100,100,100,80),
        normal_color: Color::new(200,200,200,100),
        hovered_color: Color::new(230,230,230,120),
        pressed_color: Color::new(150,150, 150, 150)
    };

    // Red Button Style
    pub const STYLE_RED: Self = Self {
        disabled_color: Color::new(160,148,145,255),
        normal_color: Color::new(255,107,107,255),
        hovered_color: Color::new(255,140,140,255),
        pressed_color: Color::new(210,77, 77, 255)
    };
}