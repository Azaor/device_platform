pub mod egui_app;
pub mod device_manager;

#[derive(Default)]
pub enum LoadingStatus {
    #[default]
    NotStarted,
    InProgress,
    Failed(String),
}
