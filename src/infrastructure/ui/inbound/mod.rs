pub mod egui_app;
pub mod device_manager;

#[derive(Default, Debug)]
pub enum LoadingStatus {
    #[default]
    NotStarted,
    InProgress,
    Failed(String),
}
