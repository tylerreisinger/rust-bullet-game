use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CommandDirectory<'a> {
    commands: HashMap<&'a str, u32>,
    commands_rev: HashMap<u32, String>,
    next_id: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command<'a> {
    id: u32,
    name: &'a str,
}

impl<'a> CommandDirectory<'a> {
    pub fn new() -> CommandDirectory<'a> {
        CommandDirectory {
            commands: HashMap::new(),
            commands_rev: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn register_command(&'a mut self, name: String) -> Command<'a> {
        let id = self.next_id;
        self.next_id += 1;

        self.commands_rev.insert(id, name);
        let name_str = &self.commands_rev[&id];
        self.commands.insert(name_str, id);

        Command { id, name: name_str }
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn get_command_by_name(&'a self, name: &str) -> Option<Command<'a>> {
        if let Some(&id) = self.commands.get(name) {
            let name_str = &self.commands_rev[&id];
            Some(Command { id, name: name_str })
        } else {
            None
        }
    }

    pub fn get_command_by_id(&'a self, id: u32) -> Option<Command<'a>> {
        if let Some(name) = self.commands_rev.get(&id) {
            Some(Command {
                id: id,
                name: name.as_str(),
            })
        } else {
            None
        }
    }
}

impl<'a> Command<'a> {}
