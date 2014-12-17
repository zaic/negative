// TODO 
// type Revision = i64;

pub trait Persistent<T> {
    fn get_by_revision(&self, revision : i64) -> T;
    fn current_revision(&self) -> i64;

    fn undo(&mut self) -> i64;
    fn redo(&mut self) -> i64;

    fn last_revision(&self) -> T {
        self.get_by_revision(self.current_revision())
    }
}
