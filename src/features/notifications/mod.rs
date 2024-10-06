pub mod schemas;
pub mod handlers {
    mod delete_notification;
    mod get_notifications;
    mod post_notification;

    pub use delete_notification::delete_notification_handler;
    pub use get_notifications::get_notifications_handler;
    pub use post_notification::post_notification_handler;
}
