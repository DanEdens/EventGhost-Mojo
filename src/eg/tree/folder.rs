use std::sync::{Arc, RwLock};
use crate::core::Error;
use super::item::{TreeItem, TreeItemInfo};

#[derive(Debug)]
pub struct Folder {
    info: TreeItemInfo,
    children: Vec<Arc<RwLock<dyn TreeItem>>>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            info: TreeItemInfo {
                id: uuid::Uuid::new_v4(),
                name: name.to_string(),
                description: String::new(),
                enabled: true,
            },
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Arc<RwLock<dyn TreeItem>>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, id: uuid::Uuid) -> Result<(), Error> {
        if let Some(index) = self.children.iter().position(|c| {
            if let Ok(child) = c.read() {
                child.get_id() == id
            } else {
                false
            }
        }) {
            self.children.remove(index);
            Ok(())
        } else {
            Err(Error::Tree(format!("Child with id {} not found", id)))
        }
    }

    pub fn get_children(&self) -> &[Arc<RwLock<dyn TreeItem>>] {
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<Arc<RwLock<dyn TreeItem>>> {
        &mut self.children
    }
}

impl TreeItem for Folder {
    fn get_id(&self) -> uuid::Uuid {
        self.info.id
    }

    fn get_name(&self) -> &str {
        &self.info.name
    }

    fn set_name(&mut self, name: &str) {
        self.info.name = name.to_string();
    }

    fn get_description(&self) -> &str {
        &self.info.description
    }

    fn set_description(&mut self, description: &str) {
        self.info.description = description.to_string();
    }

    fn is_enabled(&self) -> bool {
        self.info.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.info.enabled = enabled;
    }

    fn execute(&mut self, event: Option<&dyn crate::core::event::Event>) -> Result<(), Error> {
        for child in &self.children {
            if let Ok(mut child) = child.write() {
                if child.is_enabled() && child.can_execute(event) {
                    child.execute(event)?;
                }
            }
        }
        Ok(())
    }

    fn can_execute(&self, event: Option<&dyn crate::core::event::Event>) -> bool {
        self.children.iter().any(|c| {
            if let Ok(child) = c.read() {
                child.is_enabled() && child.can_execute(event)
            } else {
                false
            }
        })
    }

    fn clone_item(&self) -> Box<dyn TreeItem> {
        let mut folder = Folder::new(&self.info.name);
        folder.info = self.info.clone();
        folder.children = self.children.iter().map(|c| {
            if let Ok(child) = c.read() {
                Arc::new(RwLock::new(child.clone_item()))
            } else {
                panic!("Failed to read child")
            }
        }).collect();
        Box::new(folder)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
} 