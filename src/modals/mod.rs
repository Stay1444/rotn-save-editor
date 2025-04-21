use numeric_field_editor::NumericFieldEditorState;

pub mod numeric_field_editor;

pub enum Modal {
    EditNumericField(NumericFieldEditorState),
}
