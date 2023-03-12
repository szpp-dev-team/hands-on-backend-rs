pub mod contest;
pub mod task;
pub mod user;

pub trait Updatable {
    fn update(&mut self, other: &Self);
}
