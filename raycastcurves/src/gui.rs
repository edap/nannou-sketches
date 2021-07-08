use nannou::prelude::*;
use nannou::ui::prelude::*;

pub const WIN_W: u32 = 200;
pub const WIN_H: u32 = 900;
pub const PAD: Scalar = 20.0;
const COL_W: Scalar = WIN_W as Scalar - PAD * 2.0;
const LABEL_FONT_SIZE: u32 = 12;
const DEFAULT_WIDGET_H: Scalar = 30.0;
const PAD_BOTTOM: Scalar = 3.0;

widget_ids! {
    pub struct Ids {
        wall_width,
        wall_split,
        wall_padding,
        hole_pct,
        hole_n,
        tile_count_w,
        button,
        n_caster,
        ray_width,
        rays_prob,
        max_bounces,
        collision_radius,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        animation_time,
        draw_polygon,
        draw_arrows,
        polygon_contour_weight,
        animation,
        animation_speed,
        show_walls,
        draw_tex_overlay,
        clear_interval
    }
}

pub fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(COL_W, DEFAULT_WIDGET_H)
        .down(PAD_BOTTOM)
        .label_font_size(LABEL_FONT_SIZE)
        .color(color::DARK_ORANGE)
        .label_color(color::WHITE)
        .border(0.0)
}

pub fn toggle(val: bool) -> widget::Toggle<'static> {
    widget::Toggle::new(val)
        .w_h(COL_W, DEFAULT_WIDGET_H)
        .down(PAD_BOTTOM)
        .label_font_size(LABEL_FONT_SIZE)
        .color(color::DARK_ORANGE)
        .label_color(color::WHITE)
        .border(0.0)
}

pub fn button() -> widget::Button<'static, widget::button::Flat> {
    widget::Button::new()
        .w_h(COL_W, DEFAULT_WIDGET_H)
        .down(PAD_BOTTOM)
        .label_font_size(LABEL_FONT_SIZE)
        .color(color::DARK_ORANGE)
        .label_color(color::WHITE)
        .border(0.0)
}