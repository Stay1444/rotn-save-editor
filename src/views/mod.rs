use editor::EditorState;
use pick_file::PickFileState;

pub mod editor;
pub mod pick_file;

pub enum View {
    PickFile(PickFileState),
    Editor(EditorState),
}

impl Default for View {
    fn default() -> Self {
        Self::PickFile(Default::default())
    }
}
