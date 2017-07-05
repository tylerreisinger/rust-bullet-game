use std::vec;
use std::slice;
use std::mem;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use glutin::{VirtualKeyCode, ElementState, WindowEvent};
use winit;
use chrono;
use game_time::GameTime;
use float_duration::{TimePoint, FloatDuration};

pub mod command;

#[derive(Debug, Clone, PartialEq)]
pub enum Repeat {
    NoRepeat,
    EarlyRepeat(FloatDuration),
    TextRepeat(FloatDuration),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    VirtualKey(VirtualKeyCode),
    Mouse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Character(char),
    VirtKey(VirtualKeyCode, Modifiers, Repeat),
}

#[derive(Debug, Clone)]
pub struct InputManager {
    frame_events: Vec<InputEvent>,
    keys_down: HashMap<Button, chrono::DateTime<chrono::Local>>,
    modifiers: Modifiers,
    text_repeat: FloatDuration,
}

#[derive(Debug, Clone, Default)]
pub struct InputEvents {
    events: Vec<InputEvent>,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers {
            shift: false,
            ctrl: false,
            alt: false,
            super_key: false,
        }
    }
}

impl InputEvents {
    pub fn new() -> InputEvents {
        InputEvents { events: Vec::new() }
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
        InputManager {
            frame_events: Vec::new(),
            keys_down: HashMap::new(),
            modifiers: Modifiers::new(),
            text_repeat: FloatDuration::seconds(0.25),
        }
    }

    pub fn translate_event(&mut self, evt: &WindowEvent, time: &GameTime) {
        match *evt {
            WindowEvent::KeyboardInput(state, code, virt, modifiers) => {
                self.modifiers = modifiers.into();
                if let Some(v) = virt {
                    if state == ElementState::Pressed {
                        let button = Button::VirtualKey(v);
                        let entry = self.keys_down.entry(button);
                        if let Entry::Vacant(e) = entry {
                            e.insert(time.frame_start_time());
                            let event = InputEvent::VirtKey(v, modifiers.into(), Repeat::NoRepeat);
                            self.frame_events.push(event);
                        }
                    } else {
                        let button = Button::VirtualKey(v);
                        match self.keys_down.entry(button) {
                            Entry::Vacant(_) => (),
                            Entry::Occupied(e) => {
                                e.remove();
                            }
                        }
                    }
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                self.frame_events.push(InputEvent::Character(ch));
            }
            WindowEvent::Focused(gained) if !gained => self.keys_down.clear(),
            _ => (),
        }
    }

    pub fn get_events(&mut self, time: &GameTime) -> InputEvents {
        let mut events = InputEvents::new();

        mem::swap(&mut events.events, &mut self.frame_events);

        for (k, v) in &self.keys_down {
            let duration = time.frame_start_time().float_duration_since(*v).unwrap();

            let repeat = if duration.is_zero() {
                Repeat::NoRepeat
            } else if duration < self.text_repeat {
                Repeat::EarlyRepeat(duration)
            } else {
                Repeat::TextRepeat(duration)
            };

            if repeat != Repeat::NoRepeat {
                if let Button::VirtualKey(v) = *k {
                    events
                        .events
                        .push(InputEvent::VirtKey(v, self.modifiers.clone(), repeat));
                }
            }
        }

        events
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains_key(&Button::VirtualKey(key))
    }
}

impl Default for InputManager {
    fn default() -> InputManager {
        InputManager::new()
    }
}

impl InputEvents {
    pub fn len(&self) -> usize {
        self.events.len()
    }
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    pub fn as_vec(&self) -> &Vec<InputEvent> {
        &self.events
    }
}

impl IntoIterator for InputEvents {
    type Item = InputEvent;
    type IntoIter = vec::IntoIter<InputEvent>;

    fn into_iter(self) -> vec::IntoIter<InputEvent> {
        self.events.into_iter()
    }
}

impl<'a> IntoIterator for &'a InputEvents {
    type Item = &'a InputEvent;
    type IntoIter = slice::Iter<'a, InputEvent>;

    fn into_iter(self) -> slice::Iter<'a, InputEvent> {
        (&self.events).into_iter()
    }
}
