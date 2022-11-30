extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::ptr::null;
use std::thread;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use graphics::types::Matrix2d;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;
use core::time::Duration;


const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

fn createRects(points: &Vec<(u32,bool)>) -> Vec<[f64;4]>{
    let b = WIDTH/points.len() as u32;
    points.iter().map(|point| rectangle::rectangle_by_corners(5 as f64, (HEIGHT-(point.0 * 1)) as f64,0.0,HEIGHT as f64)).collect()
    
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    data_points: Vec<(u32,bool)>,
    events: Events,
    window: Window,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let rects = createRects(&self.data_points);
        let points = &self.data_points;
        let b = WIDTH/self.data_points.len() as u32;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(color::BLACK, gl);
            for n in 0..rects.len(){
                let tranform = c.transform.trans((b*n as u32) as f64, 100.0 as f64);
                if(points[n].1 == true){
                    rectangle(color::RED, rects[n], tranform, gl);
                }else{
                    rectangle(color::WHITE, rects[n], tranform, gl);
                }
                
            }

            
        });
    }

    fn update(&mut self) {
        thread::sleep(Duration::from_millis(1));
        if let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
        }
    }

    fn sort_insertion(&mut self){
        let mut i = 1;
        while (i < self.data_points.len()){
            let mut j = i;
            while (j > 0 && self.data_points[j-1] > self.data_points[j]){
                self.data_points[j].1 = true;
                self.data_points[j-1].1 = true;
                self.data_points.swap(j, j-1);
                self.update();
                self.data_points[j].1 = false;
                self.data_points[j-1].1 = false;
                j = j -1;
                
            }
            i = i+1;
            
        }
        
    }
    fn sort_selection(&mut self){
        
        //let mut i;
        //let mut j;
    
        for i in 0..self.data_points.len(){
            
            let mut min = i;
            for j in i+1..self.data_points.len(){
                if (self.data_points[j] < self.data_points[min]){
                    min = j;
                }
            }
            if (min != i){
                
                self.data_points[i].1 = true;
                self.data_points[min].1 = true;
                self.data_points.swap(i, min);
                self.update();
                self.data_points[i].1 = false;
                self.data_points[min].1 = false;
            }
        }
    }

    fn merge_init(&mut self){
        self.data_points = self.merge_sort(self.data_points.to_vec());
        self.update();
    }

    fn merge_sort( &mut self,list: Vec<(u32, bool)>) -> Vec<(u32, bool)>{
        thread::sleep(Duration::from_millis(1));
        if list.len() <= 1{
            return list;
        }

        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut index = 0;
        for x in &list{
            if index < (list.len()/2) {
                //println!("{}", left.len());
                left.push(*x);
            }else {
                right.push(*x);
            }
            index = index + 1;
        }
        left = self.merge_sort(left);
        right = self.merge_sort(right);
        let res = self.merge(left, right);
        self.data_points = res.to_vec();
        self.update();
        return res;
    }

    fn merge(&mut self, mut left: Vec<(u32, bool)>, mut right: Vec<(u32, bool)>) -> Vec<(u32, bool)>{
        let mut result = Vec::new();
        //println!("{}", result.len());
        while (left.len() >= 1 && right.len() >= 1){
            
            if (left[0] <= right[0]){
                
                result.push(left[0]);
                left.remove(0);
            }else {
                
                result.push(right[0]);
                right.remove(0);
            }
        }
        while (left.len() >= 1){
            result.push(left[0]);
            left.remove(0);
        }
        while (right.len() >= 1){
            result.push(right[0]);
            right.remove(0);
        }
        return result;
    }

    

}



fn generatePoints()-> Vec<(u32,bool)>{
    let size = 1000;
    let range = 50;
    let mut data: Vec<(u32,bool)> = Vec::new();
    let mut rng = rand::thread_rng();
    for n in (0..size){
        data.push((rng.gen_range(0..size),false));
    }
    return data;

    


}










fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut events = Events::new(EventSettings::new());
    let mut app = App {
        gl: GlGraphics::new(opengl),
        data_points: generatePoints(),
        events: events,
        window: window,
    };
    app.update();
    app.data_points = generatePoints();
    app.merge_init();
    thread::sleep(Duration::from_millis(1000));
    app.data_points = generatePoints();
    app.sort_selection();
    app.data_points = generatePoints();
    thread::sleep(Duration::from_millis(1000));
    app.data_points = generatePoints();
    app.sort_insertion();
    
    
    while let Some(e) = events.next(&mut app.window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
            //app.sort_insertion();
        }

        //if let Some(args) = e.update_args() {
        //    app.update(&args);
        //}
    }
}

#[test]
fn insert_sort_test(){
    let unsorted = generatePoints();
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut events = Events::new(EventSettings::new());
    let mut app = App {
        gl: GlGraphics::new(opengl),
        data_points: unsorted.to_vec(),
        events: events,
        window: window,
    };
    assert_eq!(unsorted.len(), app.data_points.len());
    
}