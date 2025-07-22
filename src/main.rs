use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod models;
mod processors;
mod utils;

use processors::{GhostExporter, MarkdownProcessor};
use utils::FileOps;

#[derive(Parser)]
#[command(name = "gmi")]
#[command(about = "Convert Markdown files to Ghost CMS import format", long_about = None)]
struct Cli {
    /// Input directory containing markdown files
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output file path (JSON or ZIP)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Export format
    #[arg(short, long, value_enum, default_value_t = Format::Json)]
    format: Format,

    /// Process directories recursively
    #[arg(short, long)]
    recursive: bool,

    /// Default author name for posts without authors
    #[arg(long)]
    author: Option<String>,

    /// Default tags to add to all posts
    #[arg(long, value_delimiter = ',')]
    default_tags: Vec<String>,

    /// Exclude files matching these patterns
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<String>,

    /// Include images in ZIP export
    #[arg(long)]
    include_images: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Json,
    Zip,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        anyhow::bail!("Input directory does not exist: {:?}", cli.input);
    }

    let output_path = cli.output.unwrap_or_else(|| {
        let extension = match cli.format {
            Format::Json => "json",
            Format::Zip => "zip",
        };
        std::env::current_dir()
            .unwrap()
            .join(format!("ghost-import.{}", extension))
    });

    if cli.verbose {
        println!("Searching for markdown files in: {:?}", cli.input);
    }

    let markdown_files = MarkdownProcessor::collect_markdown_files(&cli.input, 
        cli.recursive
    )?;

    if markdown_files.is_empty() {
        anyhow::bail!("No markdown files found in: {:?}", cli.input);
    }

    if cli.verbose {
        println!("Found {} markdown files", markdown_files.len());
    }

    let mut processed_posts = Vec::new();
    for file in markdown_files {
        if cli.verbose {
            println!("Processing: {:?}", file);
        }

        if let Some(file_name) = file.to_str() {
            if cli.exclude.iter().any(|pattern| {
                file_name.contains(pattern)
            }) {
                if cli.verbose {
                    println!("Skipping excluded file: {:?}", file);
                }
                continue;
            }
        }

        match MarkdownProcessor::process_file(&file) {
            Ok(processed) => {
                processed_posts.push(processed);
            }
            Err(e) => {
                eprintln!("Error processing {:?}: {}", file, e);
                continue;
            }
        }
    }

    if processed_posts.is_empty() {
        anyhow::bail!("No posts could be processed successfully");
    }

    let ghost_import = GhostExporter::create_export(
        processed_posts,
        cli.author.as_deref(),
        cli.default_tags,
    )?;

    match cli.format {
        Format::Json => {
            let json = GhostExporter::to_json(&ghost_import)?;
            std::fs::write(&output_path, json)?;
            println!("Ghost import JSON saved to: {:?}", output_path);
        }
        Format::Zip => {
            create_zip_export(&ghost_import, &cli.input, &output_path, cli.include_images, cli.verbose)?;
            println!("Ghost import ZIP saved to: {:?}", output_path);
        }
    }

    println!("Successfully processed {} posts", ghost_import.data.posts.len());
    println!("Total tags: {}", ghost_import.data.tags.len());
    println!("Total users: {}", ghost_import.data.users.len());

    Ok(())
}

fn create_zip_export(
    ghost_import: &models::ghost::GhostImport,
    input_dir: &std::path::Path,
    output_path: &std::path::Path,
    include_images: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    use std::io::Write;
    use zip::write::FileOptions;

    let file = std::fs::File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // Add JSON file
    let json = GhostExporter::to_json(ghost_import)?;
    zip.start_file("ghost-import.json", options)?;
    zip.write_all(json.as_bytes())?;

    // Add images if requested
    if include_images {
        let image_files = utils::FileOps::find_image_files(input_dir, true)?;
        
        if verbose {
            println!("Including {} images in ZIP", image_files.len());
        }

        for image_file in image_files {
            let relative_path = utils::FileOps::get_relative_path(&image_file, input_dir)?;
            let zip_path = PathBuf::from("content/images").join(relative_path);
            
            if verbose {
                println!("Adding image: {:?} -> {:?}", image_file, zip_path);
            }

            zip.start_file(zip_path.to_string_lossy(), options)?;
            let image_data = std::fs::read(image_file)?;
            zip.write_all(&image_data)?;
        }
    }

    zip.finish()?;
    Ok(())
}
