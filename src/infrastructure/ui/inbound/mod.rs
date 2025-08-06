pub mod egui_app;
pub mod panels;
pub mod managers;
pub mod widgets;

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub enum LoadingStatus {
    #[default]
    NotStarted,
    InProgress,
    Failed(String),
}
