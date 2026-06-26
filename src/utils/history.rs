use ratatui::text::Line;

#[derive(Debug)]
pub(crate) struct MoveRecord;

impl MoveRecord {
    pub(crate) fn line(&self) -> Line<'static> {
        Line::from("...")
    }
}

#[derive(Default, Debug)]
pub(crate) struct History {
    records: Vec<MoveRecord>,
}

impl History {
    pub(crate) fn lines(&self) -> Vec<Line<'static>> {
        if self.records.is_empty() {
            vec![Line::from("No moves")]
        } else {
            self.records.iter().map(MoveRecord::line).collect()
        }
    }
}
