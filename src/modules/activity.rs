use crate::{ActivityData, AppWindow, FilterBy, Log, Timeline};
use slint::ComponentHandle;
use std::{cell::RefCell, rc::Rc};

pub struct Activity {
    window: AppWindow,
}

impl Activity {
    //==============================
    //
    //==============================
    pub fn init(self) {
        let this: Rc<RefCell<Activity>> = Rc::new(RefCell::new(self));

        let window: &AppWindow = &this.borrow().window;
        let activity: ActivityData<'_> = window.global::<ActivityData>();

        //
        activity.on_clear_log({
            let this_clone: Rc<RefCell<Activity>> = this.clone();
            move || {
                //
            }
        });

        //
        activity.on_export_log({
            let this_clone: Rc<RefCell<Activity>> = this.clone();
            move || {
                //
            }
        });

        //
        activity.on_update_log({
            let this_clone: Rc<RefCell<Activity>> = this.clone();
            move |data: Log| {
                update_log(data);
            }
        });

        //
        activity.on_clear_timeline({
            let this_clone: Rc<RefCell<Activity>> = this.clone();
            move || {
                //
            }
        });

        //
        activity.on_filter_timeline({
            let this_clone: Rc<RefCell<Activity>> = this.clone();
            move |filter_by: FilterBy| {
                //
            }
        });

        //
        activity.on_update_timeline({
            let this_clone: Rc<RefCell<Activity>> = this.clone();
            move |data: Timeline| {
                update_timeline(data);
            }
        });
    }

    //==============================
    //
    //==============================
    fn update_log(&self, data: LLog) {
        //
    }

    //==============================
    //
    //==============================
    fn update_timeline(&self, data: Timeline) {
        //
    }

    //==============================
    //
    //==============================
    pub fn new(window: AppWindow) -> Self {
        Self { window }
    }
}
