use crate::{AppWindow, Cleaner, CleanersData};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::{cell::RefCell, rc::Rc};

pub struct Schema {
    window: AppWindow,
    cleaners: Vec<Cleaner>,
}

impl Schema {
    //==============================
    //
    //==============================
    pub fn init(self) {
        let this: Rc<RefCell<Schema>> = Rc::new(RefCell::new(self));

        let window: &AppWindow = &this.borrow().window;
        let cleaner: CleanersData<'_> = window.global::<CleanersData>();
        let brw_cln: VecModel<Cleaner> = VecModel::from(this.borrow().cleaners.clone());
        let model: slint::ModelRc<Cleaner> = ModelRc::from(Rc::new(brw_cln));

        // Update ui data with cleaners config list
        cleaner.set_cleaners(model);

        // Create new Cleaner config
        cleaner.on_add_new_cleaner({
            let this_clone: Rc<RefCell<Schema>> = this.clone();
            move |value: SharedString| {
                this_clone.borrow().create_new_cleaner(value.into());
            }
        });

        // Edit existing Cleaner config
        cleaner.on_edit_existing_cleaner({
            let this_clone: Rc<RefCell<Schema>> = this.clone();
            move |value: Cleaner| {
                this_clone.borrow().edit_existing_cleaner(value.into());
            }
        });
    }

    //==============================
    //
    //==============================
    fn create_new_cleaner(&self, value: String) {}

    //==============================
    //
    //==============================
    fn edit_existing_cleaner(&self, value: Cleaner) {}

    //=================
    //
    //=================
    pub fn new(window: AppWindow, cleaners: Vec<Cleaner>) -> Self {
        Self { window, cleaners }
    }
}
