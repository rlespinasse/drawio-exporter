use crate::ops::progress::{ExportEvent, ExportProgress};

/// Plain text renderer that produces output identical to the original `println!` calls.
#[derive(Default)]
pub struct PlainProgress;

impl PlainProgress {
    pub fn new() -> Self {
        PlainProgress
    }
}

impl ExportProgress for PlainProgress {
    fn on_event(&mut self, event: ExportEvent<'_>) {
        match event {
            ExportEvent::ExportStart { .. } => {}
            ExportEvent::FileStart { path, .. } => {
                println!("+ export file : {}", path);
            }
            ExportEvent::PageStart {
                page_index,
                page_name,
                ..
            } => {
                println!("- export page {} : {}", page_index, page_name);
            }
            ExportEvent::AllPagesStart => {
                println!("- export all pages");
            }
            ExportEvent::GenerateFile { format } => {
                println!("\\ generate {} file", format);
            }
            ExportEvent::GenerateDocFile { format } => {
                println!("\\ generate {} file", format);
            }
            ExportEvent::IncludeLinks { format } => {
                println!("\\ include links in {} file", format);
            }
            ExportEvent::LinkIncluded { label, url } => {
                println!("link '{}' to {}", label, url);
            }
            ExportEvent::LinkWarning { message } => {
                println!("{}", message);
            }
            ExportEvent::FileComplete => {}
            ExportEvent::ExportComplete => {}
        }
    }
}
