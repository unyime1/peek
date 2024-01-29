mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct RequestsView(ObjectSubclass<imp::RequestsView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RequestsView {
    pub fn new() -> Self {
        Object::builder().build()
    }
}