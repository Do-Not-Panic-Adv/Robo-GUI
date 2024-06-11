use robotics_lib::world::tile::Content;
use sdl2::pixels::Color;

use crate::{
    texture_manager::{get_texture_type_from_content, TextureType},
    MainState, HEIGHT, WIDTH,
};

use super::{item::Item, scene::Scene, square::Square, text::Text};

#[derive(Debug, Clone)]

pub(crate) struct Menu {
    pub(crate) menu_type: MenuTypes,
    pub(crate) is_open: bool,
}

impl Menu {
    pub(crate) fn new(menu_type: MenuTypes) -> Self {
        Self {
            menu_type,
            is_open: false,
        }
    }
    pub(crate) fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    pub(crate) fn get_menu_type(&self) -> MenuTypes {
        self.menu_type.clone()
    }
    pub(crate) fn is_open(&self) -> bool {
        self.is_open
    }

    pub(crate) fn draw(&self, state: &mut MainState) {
        match self.menu_type {
            MenuTypes::Inventory => {
                let mut invetory_scene = Scene::new(self.menu_type.to_string(), 10);
                invetory_scene.add_element(Box::new(Text::new(
                    "BACKPACK".to_string(),
                    (WIDTH as i32 / 2 - 100, 100),
                    2.0,
                    true,
                    2,
                )));
                invetory_scene.add_element(Box::new(Square::new(
                    (100, 100),
                    (WIDTH - 100, HEIGHT - 100),
                    true,
                    true,
                    Color::RGBA(200, 100, 50, 200),
                    1,
                )));
                let mut y = 150;
                let mut x = 100;
                for item in state.backpack.iter() {
                    invetory_scene.add_element(Box::new(Item::new(
                        (x + 50, y),
                        1.7,
                        true,
                        get_texture_type_from_content(item.0.clone()),
                        2,
                    )));

                    invetory_scene.add_element(Box::new(Text::new(
                        format!(
                            "{:?} x{}",
                            get_texture_type_from_content(item.0.clone()).to_string(),
                            item.1
                        ),
                        (x, y + 50),
                        0.7,
                        true,
                        1,
                    )));

                    y += 100;
                    if y > HEIGHT as i32 - 150 {
                        x += 300;
                        y = 150;
                    }
                }
                invetory_scene.draw(state);
            }

            MenuTypes::Markers => {
                let mut markers_menu = Scene::new(self.menu_type.to_string(), 9);
                // add a random string to the inventory scene

                //center text saing markers
                markers_menu.add_element(Box::new(Text::new(
                    "MARKERS".to_string(),
                    (WIDTH as i32 / 2 - 100, 100),
                    2.0,
                    true,
                    2,
                )));
                markers_menu.add_element(Box::new(Square::new(
                    (100, 100),
                    (WIDTH - 100, HEIGHT - 100),
                    true,
                    true,
                    Color::RGBA(100, 200, 50, 200),
                    1,
                )));
                let mut y = 250;
                let mut x = 100;
                let mut markers = state.markers.get_all().clone();
                for marker in markers {
                    let marked_tile =
                        state.tiles_world[marker.0 .0 as usize][marker.0 .1 as usize].clone();
                    match marked_tile {
                        Some(tile) => {
                            markers_menu.add_element(Box::new(Text::new(
                                format!("{:?}", marker.0),
                                (x, y),
                                0.7,
                                true,
                                2,
                            )));
                            markers_menu.add_element(Box::new(Item::new(
                                (x + 100, y),
                                0.7,
                                true,
                                TextureType::Tile(tile.tile_type),
                                2,
                            )));
                            markers_menu.add_element(Box::new(Item::new(
                                (x + 130, y),
                                0.7,
                                true,
                                get_texture_type_from_content(tile.content.clone()),
                                2,
                            )));
                            markers_menu.add_element(Box::new(Text::new(
                                format!("({:?}-{:?})", tile.tile_type, tile.content),
                                (x + 160, y),
                                0.7,
                                true,
                                2,
                            )));
                        }
                        None => markers_menu.add_element(Box::new(Text::new(
                            format!("{:?} - ({})", marker.0, "Unknown"),
                            (x, y),
                            0.7,
                            true,
                            2,
                        ))),
                    }

                    y += 50;
                    if y > HEIGHT as i32 - 150 {
                        x += 400;
                        y = 250;
                    }
                }

                markers_menu.draw(state);
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub(crate) enum MenuTypes {
    Inventory,
    Markers,
}
impl MenuTypes {
    pub(crate) fn to_string(&self) -> String {
        match self {
            MenuTypes::Inventory => "inventory".to_string(),
            MenuTypes::Markers => "markers".to_string(),
        }
    }
}
