use web_sys::{Element, HtmlElement, Node};
use crate::global::window::web_sys_window;

#[derive(Clone)]
pub struct DocumentData {
    pub title: String,
}

impl DocumentData {
    /// Instantiate document data using values from web_sys.
    pub(crate) fn from_web(doc: &web_sys::Document) -> Self {
        DocumentData {
            title: doc.title(),
        }
    }
}

pub struct Document {
    original: DocumentData,
    modifiable: DocumentData,

    inner: web_sys::Document,
}

impl Document {
    pub(crate) fn global() -> Self {
        let web_sys_doc = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");
        let doc = DocumentData::from_web(&web_sys_doc);
        Self {
            original: doc.clone(),
            modifiable: doc,

            inner: web_sys_doc,
        }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<crate::dom::Element> {
        web_sys_window().document().expect("Failed to get document")
            .get_element_by_id(id)
            .map(crate::dom::Element::new)
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

    pub fn body(&self) -> HtmlElement {
        self.inner.body()
            .expect("web_sys::Document::body shouldn't fail.")
    }
}

impl std::ops::Deref for Document {
    type Target = DocumentData;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

impl std::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modifiable
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        if self.modifiable.title != self.original.title {
            println!("Document title changed from {} to {}", self.original.title, self.modifiable.title);
            self.inner.set_title(&self.modifiable.title);
        }
    }
}

/// The global function to get the document.
pub fn document() -> Document {
    Document::global()
}