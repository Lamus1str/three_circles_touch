extern crate speedy2d;

use std::cmp::{max, max_by};
use speedy2d::color::Color;
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use speedy2d::dimen::Vec2;

fn main()
{

    let window = Window::new_centered("Circles", (640, 640)).unwrap();

    window.run_loop(MyWindowHandler { normal: Vec2::new(0.0, 1.0), big_r: 150.0, r: 100.0 })
}

struct MyWindowHandler
{
    normal: Vec2,
    big_r: f32,
    r: f32,
}

fn maxf(a: f32, b: f32) -> f32{
    if a > b{ a }
    else { b }
}

impl WindowHandler for MyWindowHandler
{
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        let local_pos = position - helper.get_size_pixels().into_f32() * 0.5 + Vec2::new_y(self.big_r - self.r);
        if let Some(normal) = local_pos.normalize(){
            self.normal = normal;
            helper.request_redraw();
        }
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::BLACK);
        let center = helper.get_size_pixels().into_f32() * 0.5;
        graphics.draw_circle(center, self.big_r, Color::from_gray(0.33));
        graphics.draw_circle(center - Vec2::new_y(self.big_r - self.r), self.r, Color::from_gray(0.66));
        let delta_r = self.big_r - self.r;
        let a = delta_r.powi(2) + self.r.powi(2) - 2.0 * self.normal.y * delta_r * self.r - self.big_r.powi(2);
        let b = self.normal.y * 2.0 * delta_r - 2.0 * self.big_r - 2.0 * self.r;

        let r2 = a / b;
        println!("{r2}");
        graphics.draw_circle(center + Vec2::new_y(-self.big_r + self.r) + self.normal * (self.r + r2), r2, Color::WHITE);

        //graphics.draw_circle(center + Vec2::new_y(-self.big_r + self.r) + self.normal * (self.r + r22), r22, Color::RED);
    }
}
