#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct HighlightMode {
    current: bool,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum Status {
    #[default]
    None,
    Capturable,
    Movable,
}

impl HighlightMode {
    pub(crate) fn current(&self) -> bool {
        self.current
    }

    pub(crate) fn status(&self) -> Status {
        self.status
    }

    pub(crate) fn new(current: bool, status: Status) -> Self {
        Self { current, status }
    }

    pub(crate) fn set_current(&mut self, current: bool) {
        self.current = current;
    }

    pub(crate) fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
