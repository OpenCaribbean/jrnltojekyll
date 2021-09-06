use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    pub(crate) fn open_settings(mut self) -> Self {
        self.writer
            .write(
                XmlEvent::start_element(