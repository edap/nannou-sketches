use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;

mod bouncing;
pub use crate::bouncing::BouncingRay2D;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Copy, Clone)]
struct Square {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Model {
    walls: Vec<Vector2>,
    tile_count_w: u32,
    wall_mode: u32,
    rays: Vec<BouncingRay2D>,
    draw_gui: bool,
    ui: Ui,
    ids: Ids,
    ray_width: f32,
    wall_width: f32,
    collision_radius: f32,
    rotation: f32,
    scheme_id: usize,
    max_bounces: usize,
    blend_id: usize,
    color_off: usize,
    palette: Palette,
    show_walls: bool,
    draw_tex_overlay: bool,
    animation: bool,
    animation_speed: f32,
    draw_refl: bool,
    draw_polygon: bool,
    polygon_contour_weight: f32,
    texture: wgpu::Texture,
}

widget_ids! {
    struct Ids {
        wall_width,
        tile_count_w,
        button,
        wall_mode,
        ray_width,
        max_bounces,
        collision_radius,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        draw_refl,
        draw_polygon,
        polygon_contour_weight,
        animation,
        animation_speed,
        show_walls,
        draw_tex_overlay
    }
}

fn model(app: &App) -> Model {
    let tile_count_w = 6;
    app.new_window()
        .size(900, 900)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut walls: Vec<Vector2> = Vec::new();
    let mut rays: Vec<BouncingRay2D> = Vec::new();
    let win = app.window_rect();

    let draw_gui = true;

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let ray_width = 6.0;
    let wall_width = 2.0;
    let wall_mode = 4;
    let max_bounces = 20;
    let rotation = 0.0;
    let collision_radius = 3.0;

    let scheme_id = 5;
    let blend_id = 0;
    let color_off = 4;
    let palette = Palette::new();
    make_walls(&mut walls, &mut rays, &win, tile_count_w, wall_mode);
    let show_walls = true;
    let animation = true;
    let animation_speed = 0.01;
    let draw_refl = true;
    let draw_polygon = true;
    let polygon_contour_weight = 5.0;
    let draw_tex_overlay = false;

    // texture
    // Load the image from disk and upload it to a GPU texture.
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("noise-texture1-tr.png");
    //let img_path = assets.join("images").join("grunge-halftone-tr.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        walls,
        wall_mode,
        tile_count_w,
        rays,
        max_bounces,
        draw_gui,
        ui,
        ids,
        wall_width,
        collision_radius,
        ray_width,
        rotation,
        scheme_id,
        blend_id,
        color_off,
        palette,
        show_walls,
        animation,
        animation_speed,
        draw_refl,
        draw_polygon,
        polygon_contour_weight,
        draw_tex_overlay,
        texture,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();
    {
        fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
            widget::Slider::new(val, min, max)
                .w_h(200.0, 30.0)
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.0)
        }

        fn toggle(val: bool) -> widget::Toggle<'static> {
            widget::Toggle::new(val)
                .w_h(200.0, 30.0)
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.0)
        }

        for value in slider(model.wall_width as f32, 1.0, 15.0)
            .top_left_with_margin(20.0)
            .label("wall width")
            .set(model.ids.wall_width, ui)
        {
            model.wall_width = value;
        }

        for value in slider(model.wall_mode as f32, 1.0, 4.0)
            .down(10.0)
            .label("wall_mode ")
            .set(model.ids.wall_mode , ui)
        {
            model.wall_mode  = value as u32;
        }

        for value in slider(model.tile_count_w as f32, 1.0, 6.0)
            .down(10.0)
            .label("tile_count_w")
            .set(model.ids.tile_count_w, ui)
        {
            model.tile_count_w = value as u32;
        }

        for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Regenerate Walls")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.button, ui)
        {
            let win = _app.window_rect();
            make_walls(&mut model.walls, &mut model.rays, &win, model.tile_count_w, model.wall_mode);
        }

        for value in slider(model.collision_radius as f32, 3.0, 85.0)
            .down(10.0)
            .label("collision radius")
            .set(model.ids.collision_radius, ui)
        {
            model.collision_radius = value;
        }

        for value in slider(model.ray_width, 1.0, 10.0)
            .down(10.0)
            .label("ray width")
            .set(model.ids.ray_width, ui)
        {
            model.ray_width = value;
        }
        for value in slider(model.max_bounces as f32, 1.0, 200.0)
            .down(10.0)
            .label("max_bounces")
            .set(model.ids.max_bounces, ui)
        {
            model.max_bounces = value as usize;
        }

        for val in slider(model.rotation, -PI, PI)
            .down(10.0)
            .label("Rotation")
            .set(model.ids.rotation, ui)
        {
            model.rotation = val;
        }

        for value in slider(model.scheme_id as f32, 0.0, 5.0)
            .down(10.0)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
        }

        for value in slider(model.blend_id as f32, 0.0, 3.0)
            .down(10.0)
            .label("blend_id")
            .set(model.ids.blend_id, ui)
        {
            model.blend_id = value as usize;
        }

        for value in slider(model.color_off as f32, 0.0, 4.0)
            .down(10.0)
            .label("color_off")
            .set(model.ids.color_off, ui)
        {
            model.color_off = value as usize;
        }

        for v in toggle(model.draw_refl as bool)
            .label("Draw reflection")
            .set(model.ids.draw_refl, ui)
        {
            model.draw_refl = v;
        }

        for value in slider(model.polygon_contour_weight, 1.0, 30.0)
            .down(10.0)
            .label("polygon cont weight")
            .set(model.ids.polygon_contour_weight, ui)
        {
            model.polygon_contour_weight = value;
        }

        for v in toggle(model.draw_polygon as bool)
            .label("Draw poly")
            .set(model.ids.draw_polygon, ui)
        {
            model.draw_polygon = v;
        }

        for v in toggle(model.draw_tex_overlay as bool)
            .label("Draw Overlay")
            .set(model.ids.draw_tex_overlay, ui)
        {
            model.draw_tex_overlay = v;
        }

        for v in toggle(model.animation as bool)
            .label("Animation")
            .set(model.ids.animation, ui)
        {
            model.animation = v;
        }

        for value in slider(model.animation_speed as f32, 0.1, 0.0001)
            .down(10.0)
            .label("animation speed")
            .set(model.ids.animation_speed, ui)
        {
            model.animation_speed = value;
        }

        for v in toggle(model.show_walls as bool)
            .label("Show wall")
            .set(model.ids.show_walls, ui)
        {
            model.show_walls = v;
        }
    }

    for r in model.rays.iter_mut() {
        r.max_bounces = model.max_bounces;
        r.collisions.clear();
        r.reflections.clear();
        r.refl_intensity.clear();

        // this two are not necessary but add a line more from the ray to the destination
        // r.collisions.push(r.ray.orig);
        // r.reflections.push(r.ray.dir);
        // r.refl_intensity.push(0.0);

        while !r.max_bounces_reached() {
            let mut collision: Vector2 = vec2(0.0, 0.0);
            let mut distance: f32 = Float::infinity();
            let mut surface_normal: Vector2 = vec2(0.0, 0.0);
            // find the closest intersection point between the ray and the walls
            for index in (0..model.walls.len()).step_by(2) {
                if let Some(collision_distance) = r.ray.intersect_segment(
                    model.walls[index].x,
                    model.walls[index].y,
                    model.walls[index + 1].x,
                    model.walls[index + 1].y,
                ) {
                    if collision_distance < distance {
                        distance = collision_distance;
                        collision = r.ray.orig + r.ray.dir.with_magnitude(collision_distance);
                        let segment_dir = (model.walls[index] - model.walls[index + 1]).normalize();
                        surface_normal = vec2(segment_dir.y, -segment_dir.x);
                    }
                }
            }
            if distance < Float::infinity() {
                // collision point
                r.bounces += 1;
                let refl = r.ray.reflect(surface_normal);
                r.refl_intensity.push(r.ray.dir.dot(refl).abs());
                r.ray.orig = collision + refl.with_magnitude(0.03);
                r.ray.dir = refl;
                r.collisions.push(collision);
                //r.refractions.push(r.ray.refract(surface_normal, 1.0));
                r.reflections.push(refl);
            } else {
                break;
            };
        }
        r.reset();
        if model.animation {
            r.ray.dir = r.ray.dir.rotate(_app.time * model.animation_speed);
        } else {
            r.ray.dir = r.ray.dir.rotate(model.rotation);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let blends = [BLEND_NORMAL, BLEND_ADD, BLEND_SUBTRACT, BLEND_LIGHTEST];
    let draw = app.draw().color_blend(blends[model.blend_id].clone());
    frame.clear(model.palette.get_scheme(model.scheme_id)[4]);
    //let draw = app.draw();
    draw.background()
        .color(model.palette.get_fifth(model.scheme_id, model.color_off));

    // draw the walls
    if model.show_walls {
        let size = model.walls.len();
        for index in (0..size).step_by(2) {
            draw.line()
                .weight(model.wall_width)
                .color(model.palette.get_second(model.scheme_id, model.color_off))
                .start(model.walls[index])
                .caps_round()
                .end(model.walls[index + 1]);
        }
    }

    for r in &model.rays {
        draw.arrow()
            .color(model.palette.get_first(model.scheme_id, model.color_off))
            .start(r.ray.orig)
            .stroke_weight(model.ray_width)
            .end(r.ray.orig + r.ray.dir.with_magnitude(40.0) );
        for (&c, &i) in r.collisions.iter().zip(r.refl_intensity.iter()) {
            draw.ellipse()
                .no_fill()
                .stroke(model.palette.get_third(model.scheme_id, model.color_off))
                .stroke_weight(3.0)
                .x_y(c.x, c.y)
                .w_h(model.collision_radius * i, model.collision_radius * i);
        }

        let mut col = rgba(0.0, 0.0, 0.0, 0.0);
        let ppp = r
            .collisions
            .iter()
            .zip(r.reflections.iter())
            .map(|(&co, &re)| {
                if re.x > 0.0 {
                    col = model.palette.get_third(model.scheme_id, model.color_off)
                } else {
                    col = model.palette.get_fourth(model.scheme_id, model.color_off)
                }

                (pt2(co.x, co.y), col)
            });

        if model.draw_polygon {
            draw.polygon()
                //.stroke(model.palette.get_third(model.scheme_id, model.color_off))
                .stroke(model.palette.get_second(model.scheme_id, model.color_off))
                .stroke_weight(model.polygon_contour_weight)
                .join_round()
                .points_colored(ppp);
        };

        draw.path()
            .stroke()
            .stroke_weight(model.ray_width)
            .caps_round()
            .points(r.collisions.iter().cloned())
            .color(model.palette.get_first(model.scheme_id, model.color_off));

        for (&c, &r) in r.collisions.iter().zip(r.reflections.iter()) {
            draw.arrow()
                .start(c)
                .end(c + r.with_magnitude(40.0))
                .stroke_weight(model.ray_width)
                .color(model.palette.get_first(model.scheme_id, model.color_off));
        }
    }
    if model.draw_tex_overlay {
        draw.texture(&model.texture).w_h(800.0, 800.0);
    }
    draw.to_frame(app, &frame).unwrap();

    if model.draw_gui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

fn make_walls(
    walls: &mut Vec<Vector2>,
    rays: &mut Vec<BouncingRay2D>,
    win: &geom::Rect,
    tile_count_w: u32,
    mode: u32, // 0 even, 1 random rotation, 2 one in the middle, 4 diamond
) {
    walls.clear();
    rays.clear();
    let side = win.w() as u32 / tile_count_w;
    let mut xpos = win.left();
    let mut ypos = win.bottom();

    for _x in 0..tile_count_w {
        for _y in 0..(win.h() as u32 / side as u32) {
            let coin = random_range(0.0, 1.0);
            let start_p;
            let end_p;
            let padding = 0.1 * side as f32;

            match mode {
                1 => {
                    if coin > 0.4 {
                        start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + padding);
                    } else {
                        start_p = vec2(xpos + padding, ypos + padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
                    }
                    if _x % 2 == 0 && _y % 2 == 0 {
                        let mut r = BouncingRay2D::new();
                        r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
                        r.primary_ray.orig = start_p;
                        r.ray.orig = start_p;
                        rays.push(r);
                    } else {
                        walls.push(start_p);
                        walls.push(end_p);
                    }
                }
                2 => {
                    if coin > 0.5 {
                        start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + padding);
                    } else {
                        start_p = vec2(xpos + padding, ypos + padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
                    }
                    if (_x == 2 && _y == 2) || (_x == 14 && _y == 14) {
                        let mut r = BouncingRay2D::new();
                        r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
                        r.primary_ray.orig = start_p;
                        r.ray.orig = start_p;
                        rays.push(r);
                    } else {
                        walls.push(start_p);
                        walls.push(end_p);
                    }
                }
                3 => {
                    if _x % 2 == 0 && _y % 2 == 0 {
                        start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + padding);
                        let mut r = BouncingRay2D::new();
                        //r.primary_ray.dir = Vector2::from_angle(random_range(-PI, PI));
                        r.primary_ray.dir = Vector2::from_angle(1.0);
                        // r.primary_ray.orig = start_p;
                        // r.ray.orig = start_p;
                        let o = vec2(xpos + side as f32 / 2.0, ypos + side as f32 - padding);
                        r.primary_ray.orig = o;
                        r.ray.orig = o;
                        if coin > 0.4 {
                            rays.push(r);
                        }
                    } else if _y % 2 == 0 && _x % 2 != 0 {
                        start_p = vec2(xpos + padding, ypos + padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
                    } else if _x % 2 != 0 && _y % 2 != 0 {
                        start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + padding);
                    } else {
                        start_p = vec2(xpos + padding, ypos + padding);
                        end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
                    }
                    walls.push(start_p);
                    walls.push(end_p);
                },
                4 => {
                    //let step = win.w() as f32 / 20.0;
                    let step = 200;
                    let mut squares: Vec<Square> = Vec::new();
                    squares.push(Square{
                        x: 0.0,
                        y: 0.0,
                        width: win.w() as f32,
                        height: win.h() as f32,
                    });
                    println!("{:?}", win.left());
                    println!("{:?}", win.right());
                    for i in (win.left() as i8 /2..win.right() as i8 / 2).step_by(step){
                        println!("{:?}", i);
                        split_squares(Some(i as f32), None, &mut squares);
                        split_squares(None,Some(i as f32), &mut squares);
                    }
                    for square in squares{
                        create_wall_from_square(square, walls);
                    }
                    // println!("{:?}", walls.len());
                    // println!("{:?}", walls);
                }
                _ => {}
            }

            ypos += side as f32;
        }
        ypos = win.bottom();
        xpos += side as f32;
    }
}

fn create_wall_from_square(square:Square, walls: &mut Vec<Vector2>){
    let padding = square.width * 0.1;
    walls.push(vec2(square.x, square.y));
    walls.push(vec2(square.x + square.width - padding, square.y));

}

fn split_squares(x: Option<f32>, y: Option<f32>, squares: &mut Vec<Square> ){
    for i in (0..squares.len()).rev(){
        let square = squares[i].clone();
        if let Some(x_val) = x{
            println!("{}", x_val);
            if x_val > square.x && x_val < square.x + square.width {
                if random_range(0.0, 1.0) > 0.0{
                    split_on_x(i, squares, x_val);
                }
            }
        }
        // if let Some(y_val) = y{
        //     if y_val > square.y && y_val < square.y + square.height {
        //         if random_range(0.0, 1.0) > 0.5{
        //             split_on_y(i, squares, y_val);
        //         }
        //     }
        // }
    }
}

fn split_on_x(square_index: usize, squares: &mut Vec<Square>, split_at: f32) {
    let square = &squares[square_index];
    let square_a = Square{
        x: square.x,
        y: square.y,
        width: square.width - (square.width - split_at + square.x),
        height: square.height,
    };
    let square_b = Square{
        x: square.x,
        y: square.y,
        width: square.width - split_at + square.x,
        height: square.height,
    };
    // make a copy
    let mut copy_squares: Vec<Square> = Vec::new();
    for square in squares.iter_mut(){
        copy_squares.push(Square{
            x: square.x,
            y: square.y,
            width: square.width,
            height: square.height,            
        });
    }
    squares.clear();

    for i in (0..copy_squares.len()).rev(){
        if i == square_index{
            squares.push(square_a);
            squares.push(square_b);
        }else{
            squares.push(copy_squares[i]);
        }
    }
}

  fn split_on_y(square_index: usize, squares: &mut Vec<Square>, split_at: f32) {
    let square = &squares[square_index];
    let square_a = Square{
        x: square.x,
        y: square.y,
        width: square.width,
        height: square.height - (square.height - split_at + square.y),
    };
    let square_b = Square{
        x: square.x,
        y: square.y,
        width: square.width,
        height: square.height - split_at + square.y,
    };
    // make a copy
    let mut copy_squares: Vec<Square> = Vec::new();
    for square in squares.iter_mut(){
        copy_squares.push(Square{
            x: square.x,
            y: square.y,
            width: square.width,
            height: square.height,            
        });
    }
    squares.clear();


    for i in (0..copy_squares.len()).rev(){
        if i == square_index{
            squares.push(square_a);
            squares.push(square_b);
        }else{
            squares.push(copy_squares[i]);
        }
    }
    println!("{:?}", squares.len());
  }


fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
            //.capture_frame(app.exe_name().unwrap() + ".png");
        }
        Key::G => model.draw_gui = !model.draw_gui,
        _other_key => {}
    }
}


