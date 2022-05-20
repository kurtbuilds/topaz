use std::ops::{Deref, DerefMut};
use web_sys::{Element, Node};
use crate::global::window::web_sys_window;

#[derive(Clone)]
pub struct DocumentData {
    pub title: String,
}

impl DocumentData {
    /// Instantiate document data using values from web_sys.
    pub fn from_web(doc: web_sys::Document) -> Self {
        DocumentData {
            title: doc.title(),
        }
    }
}

pub struct Document {
    original: DocumentData,
    modifiable: DocumentData,
}

impl Document {
    pub(crate) fn from_web(doc: web_sys::Document) -> Self {
        let doc = DocumentData::from_web(doc);
        Self {
            original: doc.clone(),
            modifiable: doc,
        }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<Element> {
        web_sys_window().document().expect("Failed to get document")
            .get_element_by_id(id)
    }

    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<crate::dom::Element> {
        let collection = web_sys_window().document().expect("Failed to get document")
            .get_elements_by_tag_name(tag_name);
        let max = collection.length();
        (0..max)
            .map(|i| crate::dom::Element::new(
                collection.item(i).expect("Failed to get element")
            ))
            .collect()
    }
}

impl Deref for Document {
    type Target = DocumentData;

    fn deref(&self) -> &Self::Target {
        &self.modifiable
    }
}

impl DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modifiable
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        if self.modifiable.title != self.original.title {
            println!("Document title changed from {} to {}", self.original.title, self.modifiable.title);
            // console_log!("document.title was updated from {} to {}", self.original.title, self.modifiable.title);
            web_sys::window().unwrap().document().unwrap().set_title(&self.modifiable.title.clone())
        }
    }
}

/// The global function to get the document.
pub fn document() -> Document {
    let web_sys_doc = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window");
    Document::from_web(web_sys_doc)
}