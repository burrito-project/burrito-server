pub mod schemas;
pub mod rc {
    mod default_flags;

    pub use default_flags::setup_base_flags;
}
mod utils {
    pub mod get_flag;
}

pub use utils::get_flag::get_flag;
