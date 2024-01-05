use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{gio, glib, Box, CompositeTemplate, ListView};
use once_cell::sync::OnceCell;

// Initialize composite template for Window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/etim/querry/collection.ui")]
pub struct CollectionsWindow {
    #[template_child]
    pub collections_list: TemplateChild<ListView>,
    pub collections_store: OnceCell<gio::ListStore>,
    #[template_child]
    pub empty_collections_box: TemplateChild<Box>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CollectionsWindow {
    const NAME: &'static str = "QuerryCollectionsWindow";
    type Type = super::CollectionsWindow;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for CollectionsWindow {
    fn constructed(&self) {
        // Calls at the time window is constructed.
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_collections();
        obj.add_new_collection();
        obj.setup_collection_click();
        obj.calc_visible_child();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CollectionsWindow {}

impl BoxImpl for CollectionsWindow {}
