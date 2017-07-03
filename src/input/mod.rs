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
