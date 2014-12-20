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

        for _ in range(1, times) {
            self.undo();
        }
        self.undo()
    }

    fn redo_ntimes(&mut self, times: int) -> Revision {
        assert!(times > 0);

        for _ in range(1, times) {
            self.redo();
        }
        self.redo()
    }
}

pub trait FullyPersistent<T>: Persistent<T> + Recall { }
