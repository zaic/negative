// TODO 
// type Revision = i64;

pub trait Persistent<T> {
    fn get_by_revision(&self, revision : i64) -> T;
    fn current_revision_id(&self) -> i64;

    fn current(&self) -> T {
        self.get_by_revision(self.current_revision_id())
    }
}

pub trait Recall {
    fn undo(&mut self) -> i64;
    fn redo(&mut self) -> i64;
}

pub trait FullPersistent<T> : Persistent<T> + Recall {

}
