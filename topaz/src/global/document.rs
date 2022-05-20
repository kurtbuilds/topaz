use std::ops::{Deref, DerefMut};

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

    pub fn get_element_by_id(&self, id: &str) {
        todo!();
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