use nannou::prelude::*;
use nannou::ui::prelude::*;

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
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn toggle(val: bool) -> widget::Toggle<'static> {
    widget::Toggle::new(val)
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}
