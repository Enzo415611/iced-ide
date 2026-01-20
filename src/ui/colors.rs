use iced::Color;

// button border color
pub const BORDER_COLOR: Color = Color::from_rgb8(83, 85, 87); 
pub const BUTTON_BACKGROUND_COLOR: Color = Color::from_rgb8(140, 170, 238);
pub const BUTTON_HOVERED_COLOR: Color = Color::from_rgb8(48, 58, 70);


pub fn get_color_num() {
    let test = iced::Theme::CatppuccinFrappe.palette().primary;
    println!("{}", test)
}