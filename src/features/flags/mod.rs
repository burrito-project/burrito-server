pub mod schemas;
pub mod rc {
    mod default_flags;

    pub use default_flags::setup_base_flags;
}
pub mod utils {
    mod get_flag;

    pub use get_flag::get_flag;
}
