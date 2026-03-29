/// Events emitted by the export process.
pub enum ExportEvent<'a> {
    ExportStart {
        total_files: usize,
    },
    FileStart {
        path: &'a str,
        file_index: usize,
        total_files: usize,
    },
    PageStart {
        page_index: usize,
        page_name: &'a str,
        total_pages: usize,
    },
    AllPagesStart,
    GenerateFile {
        format: &'a str,
    },
    GenerateDocFile {
        format: &'a str,
    },
    IncludeLinks {
        format: &'a str,
    },
    LinkIncluded {
        label: &'a str,
        url: &'a str,
    },
    LinkWarning {
        message: String,
    },
    FileComplete,
    ExportComplete,
}

/// Trait for receiving export progress events.
pub trait ExportProgress {
    fn on_event(&mut self, event: ExportEvent<'_>);
}
