use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // Build Properties element
    // i.e. <Properties xmlns:vt="http://schemas.openxmlformats.org/package/2006/relationships">
    open!(open_properties, "Properties", "xmlns", "xmlns:vt");

    closed_with_child!(template, "Template");
    closed_with_child!(total_time, "TotalTime");
    closed_with_child!(application, "Application");
    closed_with_child!(pages, "Pages");
    closed_wit