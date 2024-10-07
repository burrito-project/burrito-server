pub mod schemas;
pub mod handlers {
    mod get_flag;
    mod list_flags;
    mod update_flag;

    pub use get_flag::get_flag_handler;
    pub use list_flags::list_flags_handler;
    pub use update_flag::update_flag_handler;
}
pub mod rc {
    mod default_flags;

    pub use default_flags::setup_base_flags;
}
mod utils {
    pub mod get_flag;
}

pub use utils::get_flag::get_flag;
