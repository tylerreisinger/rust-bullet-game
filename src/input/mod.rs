use std::collections::HashMap;

use glutin::{VirtualKeyCode, ElementState, WindowEvent};
use winit;

pub mod command;

use self::command::{Command, CommandDirectory};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputEvent {
    Character(char),
    VirtKey(VirtualKeyCode, Modifiers),
}

pub struct InputMap<'a> {
    directory: CommandDirectory<'a>,
    mapping: HashMap<InputEvent, u32>,
}

#[derive(Debug, Clone)]
pub struct InputManager {
    inputs: Vec<InputEvent>,
}

impl<'a> InputMap<'a> {
    pub fn new(directory: CommandDirectory<'a>) -> InputMap<'a> {
        InputMap {
            directory,
            mapping: HashMap::new(),
        }
    }

    pub fn map_event(&'a self, event: &WindowEvent) -> Option<Command<'a>> {
        match *event {
            WindowEvent::ReceivedCharacter(ch) => {
                self.mapping
                    .get(&InputEvent::Character(ch))
                    .and_then(|id| self.directory.get_command_by_id(*id))
            }
            WindowEvent::KeyboardInput(state, _, virt_key, modi) => {
                if state == ElementState::Pressed {
                    virt_key
                        .map(|key| InputEvent::VirtKey(key, modi.into()))
                        .and_then(|k| self.mapping.get(&k))
                        .and_then(|id| self.directory.get_command_by_id(*id))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<winit::ModifiersState> for Modifiers {
    fn from(from: winit::ModifiersState) -> Modifiers {
        Modifiers {
            shift: from.shift,
            ctrl: from.ctrl,
            alt: from.alt,
            super_key: from.logo,
        }
    }
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager { inputs: Vec::new() }
    }

    pub fn translate_event(&mut self, evt: WindowEvent) -> bool {
        let input_event = match evt {
            WindowEvent::KeyboardInput(state, code, virt, modi) => {
                println!("{:?} {} {:?} {:?}", state, code, virt, modi);
                if state == ElementState::Pressed {
                    virt.map(|v| InputEvent::VirtKey(v, modi.into()))
                } else {
                    None
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                println!("Char: {}", ch);
                Some(InputEvent::Character(ch))
            }
            _ => None,
        };

        if let Some(e) = input_event {
            self.inputs.push(e);
            true
        } else {
            false
        }
    }

    pub fn get_events(&self) -> &Vec<InputEvent> {
        &self.inputs
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
}
