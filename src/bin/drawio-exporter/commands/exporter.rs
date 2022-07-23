use anyhow::Result;
use clap::ArgMatches;

use drawio_exporter::ops::exporter::{exporter, ExporterOptions};
use drawio_exporter::util::command_prelude::*;

pub fn args() -> Vec<Arg> {
    vec![
        // Cli options
        arg("application")
            .help("Draw.io Desktop Application")
            .value_name("path")
            .short("A")
            .long("application"),
        arg("drawio-desktop-headless")
            .help("Enable Draw.io Desktop headless mode")
            .long("drawio-desktop-headless"),
        arg("format")
            .help("Exported format")
            .value_name("format")
            .possible_values(&["adoc", "jpg", "pdf", "png", "svg", "vsdx", "xml"])
            .default_value("pdf")
            .short("f")
            .long("format"),
        arg("folder")
            .help("Exported folder name")
            .value_name("folder")
            .default_value("export")
            .short("o")
            .long("output"),
        arg("on-changes")
            .help("Export drawio files only if it's newer than exported files")
            .long("on-changes"),
        arg("git-reference")
            .help("Any git reference (branch, tag, commit id, ...)")
            .value_name("reference")
            .long("git-ref")
            // if a git reference is set, always consider we only explore modified files
            // so we make the user award of that fact
            .requires("on-changes"),
        arg("remove-page-suffix")
            .help("Remove page suffix when possible (in case of single page file)")
            .long("remove-page-suffix"),
        arg("path")
            .help("Path to the drawio files to export")
            .value_name("PATH")
            .default_value(".")
            .index(1),
        // Drawio Desktop cli options for all formats
        arg("drawio-cli-border")
            .help("Sets the border width around the diagram")
            .value_name("border")
            .default_value("0")
            .short("b")
            .long("border"),
        arg("drawio-cli-scale")
            .help("Scales the diagram size")
            .value_name("scale")
            .short("s")
            .long("scale"),
        // Drawio Desktop cli options for PDF format
        arg("drawio-cli-pdf-width")
            .help("Fits the generated image/pdf into the specified width, preserves aspect ratio")
            .value_name("width")
            .long("width"),
        arg("drawio-cli-pdf-height")
            .help("Fits the generated image/pdf into the specified height, preserves aspect ratio")
            .value_name("height")
            .long("height"),
        arg("drawio-cli-pdf-crop")
            .help("crops PDF to diagram size")
            .long("crop"),
        // Drawio Desktop cli options for PNG format
        arg("drawio-cli-png-transparent")
            .help("Set transparent background for PNG")
            .short("t")
            .long("transparent"),
        // Drawio Desktop cli options for JPEG format
        arg("drawio-cli-jpg-quality")
            .help("Output image quality for JPEG")
            .value_name("quality")
            .default_value("90")
            .short("q")
            .long("quality"),
        // Drawio Desktop cli options for XML format
        arg("drawio-cli-xml-uncompressed")
            .help("Uncompressed XML output")
            .short("u")
            .long("uncompressed"),
        // Drawio Desktop cli options for SVG format
        arg("drawio-cli-svg-embed-svg-images")
            .help("Embed Images in SVG file")
            .long("embed-svg-images"),
        // Drawio Desktop cli options for multiple formats
        arg("drawio-cli-pdf-png-svg-embed-diagram")
            .help("Includes a copy of the diagram for PDF, PNG, or SVG")
            .short("e")
            .long("embed-diagram"),
    ]
}

pub fn exec(args: &ArgMatches<'_>) -> Result<()> {
    exporter(ExporterOptions {
        application: args.value_of("application"),
        drawio_desktop_headless: args.is_present("drawio-desktop-headless"),
        folder: args.value_of("folder").unwrap(),
        on_filesystem_changes: args.is_present("on-changes"),
        on_git_changes_since_reference: args.value_of("git-reference"),
        remove_page_suffix: args.is_present("remove-page-suffix"),
        path: args.value_of("path").unwrap(),
        format: args.value_of("format").unwrap(),
        border: args.value_of("drawio-cli-border").unwrap(),
        scale: args.value_of("drawio-cli-scale"),
        width: args.value_of("drawio-cli-pdf-width"),
        height: args.value_of("drawio-cli-pdf-height"),
        crop: args.is_present("drawio-cli-pdf-crop"),
        transparent: args.is_present("drawio-cli-png-transparent"),
        quality: args.value_of("drawio-cli-jpg-quality").unwrap(),
        uncompressed: args.is_present("drawio-cli-xml-uncompressed"),
        embed_svg_images: args.is_present("drawio-cli-svg-embed-svg-images"),
        embed_diagram: args.is_present("drawio-cli-pdf-png-svg-embed-diagram"),
    })
}
