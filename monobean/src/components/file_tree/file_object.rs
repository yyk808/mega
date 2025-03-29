use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use std::cell::RefCell;

    use super::*;
    use gtk::glib::Properties;

    // Object holding the state
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::TaskObject)]
    pub struct TaskObject {
        #[property(name = "completed", get, set, type = bool, member = completed)]
        #[property(name = "content", get, set, type = String, member = content)]
        pub data: RefCell<TaskData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaskObject {
        const NAME: &'static str = "TodoTaskObject";
        type Type = super::TaskObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TaskObject {}
}

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}

impl TaskObject {
    pub fn new(completed: bool, content: String) -> Self {
        glib::Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
}

#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
