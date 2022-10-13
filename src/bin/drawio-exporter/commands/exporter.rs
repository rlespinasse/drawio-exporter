use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches};
use drawio_exporter::core::drawio::drawio_desktop::os_default_application;
use drawio_exporter::ops::exporter::{exporter, ExporterOptions};

pub fn args() -> Vec<Arg> {
    vec![
        // Cli options
        Arg::new("application")
            .help("Draw.io Desktop Application")
            .value_name("path")
            .default_value(os_default_application())
            .short('A')
            .long("application"),
        Arg::new("drawio-desktop-headless")
            .help("Enable Draw.io Desktop headless mode")
            .long("drawio-desktop-headless"),
        Arg::new("format")
            .help("Exported format")
            .value_name("format")
            .value_parser(["adoc", "md", "jpg", "pdf", "png", "svg", "vsdx", "xml"])
            .default_value("pdf")
            .short('f')
            .long("format"),
        Arg::new("folder")
            .help("Exported folder name")
            .value_name("folder")
            .default_value("export")
            .short('o')
            .long("output"),
        Arg::new("on-changes")
            .help("Export drawio files only if it's newer than exported files")
            .long("on-changes")
            .action(ArgAction::SetTrue),
        Arg::new("git-reference")
            .help("Any git reference (branch, tag, commit id, ...)")
            .value_name("reference")
            .long("git-ref")
            // if a git reference is set, always consider we only explore modified files
            // so we make the user award of that fact
            .requires("on-changes"),
        Arg::new("remove-page-suffix")
            .help("Remove page suffix when possible (in case of single page file)")
            .long("remove-page-suffix")
            .action(ArgAction::SetTrue),
        Arg::new("path")
            .help("Path to the drawio files to export")
            .value_name("PATH")
            .default_value(".")
            .index(1),
        // Drawio Desktop cli options for all formats
        Arg::new("drawio-cli-border")
            .help("Sets the border width around the diagram")
            .value_name("border")
            .default_value("0")
            .short('b')
            .long("border"),
        Arg::new("drawio-cli-scale")
            .help("Scales the diagram size")
            .value_name("scale")
            .short('s')
            .long("scale"),
        Arg::new("drawio-cli-enable-plugins")
            .help("Enable Plugins")
            .long("enable-plugins")
            .action(ArgAction::SetTrue),
        // Drawio Desktop cli options for PDF format
        Arg::new("drawio-cli-pdf-width")
            .help("Fits the generated image/pdf into the specified width, preserves aspect ratio")
            .value_name("width")
            .long("width"),
        Arg::new("drawio-cli-pdf-height")
            .help("Fits the generated image/pdf into the specified height, preserves aspect ratio")
            .value_name("height")
            .long("height"),
        Arg::new("drawio-cli-pdf-crop")
            .help("crops PDF to diagram size")
            .long("crop")
            .action(ArgAction::SetTrue),
        // Drawio Desktop cli options for PNG format
        Arg::new("drawio-cli-png-transparent")
            .help("Set transparent background for PNG")
            .short('t')
            .long("transparent")
            .action(ArgAction::SetTrue),
        // Drawio Desktop cli options for JPEG format
        Arg::new("drawio-cli-jpg-quality")
            .help("Output image quality for JPEG")
            .value_name("quality")
            .default_value("90")
            .short('q')
            .long("quality"),
        // Drawio Desktop cli options for XML format
        Arg::new("drawio-cli-xml-uncompressed")
            .help("Uncompressed XML output")
            .short('u')
            .long("uncompressed")
            .action(ArgAction::SetTrue),
        // Drawio Desktop cli options for SVG format
        Arg::new("drawio-cli-svg-embed-svg-images")
            .help("Embed Images in SVG file")
            .long("embed-svg-images")
            .action(ArgAction::SetTrue),
        // Drawio Desktop cli options for multiple formats
        Arg::new("drawio-cli-pdf-png-svg-embed-diagram")
            .help("Includes a copy of the diagram for PDF, PNG, or SVG")
            .short('e')
            .long("embed-diagram")
            .action(ArgAction::SetTrue),
    ]
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    exporter(ExporterOptions {
        application: args.get_one("application").unwrap(),
        drawio_desktop_headless: args.contains_id("drawio-desktop-headless"),
        folder: args.get_one("folder").unwrap(),
        on_filesystem_changes: args.get_one::<bool>("on-changes").copied().unwrap(),
        on_git_changes_since_reference: args.get_one("git-reference"),
        remove_page_suffix: args.get_one::<bool>("remove-page-suffix").copied().unwrap(),
        path: args.get_one::<String>("path").unwrap(),
        format: args.get_one("format").unwrap(),
        border: args.get_one("drawio-cli-border").unwrap(),
        scale: args.get_one("drawio-cli-scale"),
        enable_plugins: args
            .get_one::<bool>("drawio-cli-enable-plugins")
            .copied()
            .unwrap(),
        width: args.get_one("drawio-cli-pdf-width"),
        height: args.get_one("drawio-cli-pdf-height"),
        crop: args
            .get_one::<bool>("drawio-cli-pdf-crop")
            .copied()
            .unwrap(),
        transparent: args
            .get_one::<bool>("drawio-cli-png-transparent")
            .copied()
            .unwrap(),
        quality: args.get_one("drawio-cli-jpg-quality").unwrap(),
        uncompressed: args
            .get_one::<bool>("drawio-cli-xml-uncompressed")
            .copied()
            .unwrap(),
        embed_svg_images: args
            .get_one::<bool>("drawio-cli-svg-embed-svg-images")
            .copied()
            .unwrap(),
        embed_diagram: args
            .get_one::<bool>("drawio-cli-pdf-png-svg-embed-diagram")
            .copied()
            .unwrap(),
    })
}
