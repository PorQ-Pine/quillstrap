use crate::prelude::*;
use color_eyre::eyre::WrapErr;

pub fn generate_d2_graph(output_dir: &Path) -> Result<()> {
    let mut d2_graph_content = String::new();
    let things = crate::things::get_things();

    for thing in things.iter() {
        let thing_name = thing.name();
        d2_graph_content.push_str(&format!("{}\n", thing_name));

        for dep_name in thing.deps().iter() {
            d2_graph_content.push_str(&format!("{} -> {}\n", thing_name, dep_name));
        }
        d2_graph_content.push('\n');
    }

    let d2_dir = output_dir.join("d2");
    std::fs::create_dir_all(&d2_dir)?;

    let d2_file_path = d2_dir.join("graph.d2");
    std::fs::write(&d2_file_path, d2_graph_content)?;

    info!(
        "D2 graph generated at: {}",
        d2_file_path.to_string_lossy()
    );

    let svg_file_path = d2_dir.join("graph.svg");
    let d2_file_str = d2_file_path.to_string_lossy();
    let svg_file_str = svg_file_path.to_string_lossy();

    info!("Generating SVG: {}", svg_file_str);
    run_shell_command(
        &format!("d2 --layout elk --dark-theme 200 {} {}", d2_file_str, svg_file_str),
        true, // show output
    )
    .context("Failed to generate SVG from D2 graph")?;

    Ok(())
}
