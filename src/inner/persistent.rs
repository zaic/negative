pub type Revision = int;

pub trait Persistent<T> {
    fn get_by_revision(&self, revision : Revision) -> T;
    fn current_revision_id(&self) -> Revision;

    fn current(&self) -> T {
        self.get_by_revision(self.current_revision_id())
    }
}

pub trait Recall {
    fn undo(&mut self) -> Revision;
    fn redo(&mut self) -> Revision;

    fn undo_ntimes(&mut self, times: int) -> Revision {
        assert!(times > 0);

        while times > 1) {
            undo();
        }
        undo()
    }

    fn redo_ntimes(&mut self, times: int) -> Revision {
        assert!(times > 0);

        while (times > 1) {
            redo();
        }
        redo()
    }
}

pub trait FullyPersistent<T>: Persistent<T> + Recall {}
