use nannou::prelude::*;
use nannou_conrod as ui;
use nannou_conrod::prelude::*;

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 900;
pub const PAD: Scalar = 20.0;
pub const N_COL: u32 = 4;
pub const COL_W: Scalar = (WIN_W / N_COL) as Scalar - PAD * 2.0;
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
        button_refl,
        button_diffuse,
        button_refl_refr,
        button_regenerate,
        n_caster,
        raycaster_density,
        ray_width,
        rays_prob,
        rays_position_mode,
        max_bounces,
        collision_radius,
        rotation,
        scheme_id,
        blend_id,
        palette_alpha,
        color_off,
        light_color_pct,
        animation_time,
        draw_rays,
        draw_polygon,
        draw_polygon_mode,
        draw_arrows,
        clean_bg,
        transparent_bg,
        polygon_contour_weight,
        animation,
        animation_mode,
        animation_speed,
        show_walls,
        draw_not_colliding_rays,
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

// pub fn text(content: String) -> widget::Text<'static> {
//     widget::Text::new("Hello World!")
//     .w_h(COL_W, DEFAULT_WIDGET_H)
//     .down(PAD_BOTTOM)
//     .color(color::DARK_ORANGE)
// }
