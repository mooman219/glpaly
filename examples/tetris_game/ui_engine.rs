use storm::*;
use storm::math::AABB2D;
use storm::cgmath::*;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct UIPos {
    pub x: f32,
    pub y: f32,
}

impl UIPos {
    pub fn new(x: f32, y: f32) -> UIPos {
        UIPos {
            x,
            y,
        }
    }
}

pub type UIElementToken = u32;

pub trait UIElement {
    fn draw(&mut self, sprites: &mut Vec<Sprite>, texts: &mut Vec<Text>);
    fn bounding_box(&self) -> AABB2D;
    fn click_down(&mut self, click_point: Vector2<f32>);
    fn click_up(&mut self, up_point: Vector2<f32>)->bool;
}


pub struct UIEngine {
    pub screen: BatchToken,
    ui_elements: HashMap<UIElementToken, Box<dyn UIElement>>,
    is_clicked_objects: Vec<UIElementToken>,
    ui_element_count: u32,
}

impl UIEngine {
    pub fn new(engine: &mut Engine) -> UIEngine { 
        let screen = engine.batch_create(&BatchSettings::default());

        UIEngine {
            screen,
            ui_elements: HashMap::new(),
            is_clicked_objects: vec![],
            ui_element_count: 0,
        }
    }

    pub fn add_new_ui_element(&mut self, ui_element: Box<dyn UIElement>) -> UIElementToken {
        self.ui_element_count += 1;
        self.ui_elements.insert(self.ui_element_count, ui_element);
        return self.ui_element_count;
    }

    pub fn draw(&mut self, engine: &mut Engine) {
        let mut sprites = vec![];
        let mut texts = vec![];
        for (_, element) in self.ui_elements.iter_mut() {
            element.draw(&mut sprites, &mut texts);
        }
        engine.sprite_set(&self.screen, &sprites);
        engine.text_set(&self.screen, &texts);
    }

    pub fn click_down_event(&mut self, pos: Vector2<f32>) {
        for (k, element) in self.ui_elements.iter_mut() {
            let aabb = element.bounding_box();
            if aabb.contains_point(&pos) {
                element.click_down(pos);
                self.is_clicked_objects.push(*k);
            }
        }
    }

    pub fn click_up_event(&mut self, pos: Vector2<f32>) -> Vec<UIElementToken> {
        let mut click_down_responded = vec![];
        for k in self.is_clicked_objects.iter() {
            let object = self.ui_elements.get_mut(k).unwrap();
            if object.click_up(pos) {
                click_down_responded.push(*k);
            }
        }
        self.is_clicked_objects = vec![];
        return click_down_responded;
    }
}