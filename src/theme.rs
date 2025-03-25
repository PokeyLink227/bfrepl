#![allow(unused)]
use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub root: Style,
    pub root_tab_selected: Style,
    pub command_error: Style,
    pub popup: Style,
    pub popup_focused: Style,
    pub popup_selected: Style,
    pub key_bind: Style,
    pub key_desc: Style,
    pub mode: ModeTheme,
}

pub struct ModeTheme {
    pub normal: Style,
    pub editing: Style,
    pub command: Style,
}

pub const THEME: Theme = Theme {
    root: Style::new().fg(WHITE),
    root_tab_selected: Style::new().fg(YELLOW).bg(DARK_GRAY),
    command_error: Style::new().fg(RED),
    popup: Style::new().fg(WHITE),
    popup_focused: Style::new().fg(WHITE).bg(DARK_GRAY),
    popup_selected: Style::new().fg(YELLOW).bg(DARK_GRAY),
    key_bind: Style::new().fg(BLACK).bg(DARK_GRAY),
    key_desc: Style::new().fg(DARK_GRAY).bg(BLACK),
    mode: ModeTheme {
        normal: Style::new().fg(WHITE).bg(BLUE_3),
        editing: Style::new().fg(WHITE).bg(GREEN_1),
        command: Style::new().fg(WHITE).bg(YELLOW),
    },
};

const WHITE: Color = Color::Rgb(238, 238, 238);
const LIGHT_GRAY: Color = Color::Rgb(188, 188, 188);
const GRAY: Color = Color::Rgb(128, 128, 128);
const DARK_GRAY: Color = Color::Rgb(68, 68, 68);
const DARKER_GRAY: Color = Color::Rgb(38, 38, 38);
const BLACK: Color = Color::Rgb(8, 8, 8);
const YELLOW: Color = Color::Rgb(240, 180, 30);
const RED: Color = Color::Rgb(210, 60, 60);

//from apollo color palette
const BLUE_0: Color = Color::from_u32(0x00172038);
const BLUE_1: Color = Color::from_u32(0x00253a5e);
const BLUE_2: Color = Color::from_u32(0x003c5e8b);
const BLUE_3: Color = Color::from_u32(0x004f8fba);
const BLUE_4: Color = Color::from_u32(0x0073bed3);
const BLUE_5: Color = Color::from_u32(0x00a4dddb);

const GREEN_0: Color = Color::from_u32(0x0019332d);
const GREEN_1: Color = Color::from_u32(0x0025562e);
const GREEN_2: Color = Color::from_u32(0x00468232);
const GREEN_3: Color = Color::from_u32(0x0075a743);
const GREEN_4: Color = Color::from_u32(0x00a8ca58);
const GREEN_5: Color = Color::from_u32(0x00d0da91);

const BROWN_0: Color = Color::from_u32(0x004d2b32);
const BROWN_1: Color = Color::from_u32(0x007a4841);
const BROWN_2: Color = Color::from_u32(0x007a4841);
const BROWN_3: Color = Color::from_u32(0x00c09473);
const BROWN_4: Color = Color::from_u32(0x00d7b594);
const BROWN_5: Color = Color::from_u32(0x00e7d5b3);

const YELLOW_0: Color = Color::from_u32(0x00341c27);
const YELLOW_1: Color = Color::from_u32(0x00602c2c);
const YELLOW_2: Color = Color::from_u32(0x00884b2b);
const YELLOW_3: Color = Color::from_u32(0x00be772b);
const YELLOW_4: Color = Color::from_u32(0x00de9e41);
const YELLOW_5: Color = Color::from_u32(0x00e8c170);

const RED_0: Color = Color::from_u32(0x00241527);
const RED_1: Color = Color::from_u32(0x00411d31);
const RED_2: Color = Color::from_u32(0x00752438);
const RED_3: Color = Color::from_u32(0x00a53030);
const RED_4: Color = Color::from_u32(0x00cf573c);
const RED_5: Color = Color::from_u32(0x00da863e);

const PINK_0: Color = Color::from_u32(0x001e1d39);
const PINK_1: Color = Color::from_u32(0x00402751);
const PINK_2: Color = Color::from_u32(0x007a367b);
const PINK_3: Color = Color::from_u32(0x00a23e8c);
const PINK_4: Color = Color::from_u32(0x00c65197);
const PINK_5: Color = Color::from_u32(0x00df84a5);

const GRAY_0: Color = Color::from_u32(0x00090a14);
const GRAY_1: Color = Color::from_u32(0x0010141f);
const GRAY_2: Color = Color::from_u32(0x00151d28);
const GRAY_3: Color = Color::from_u32(0x00202e37); // happens to be background on terminal
const GRAY_4: Color = Color::from_u32(0x00394a50);
const GRAY_5: Color = Color::from_u32(0x00577277);
const GRAY_6: Color = Color::from_u32(0x00819796);
const GRAY_7: Color = Color::from_u32(0x00a8b5b2);
const GRAY_8: Color = Color::from_u32(0x00c7cfcc);
const GRAY_9: Color = Color::from_u32(0x00ebede9);
