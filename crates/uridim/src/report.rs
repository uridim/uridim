use std::fmt::Display;
use std::io::{self, Write};

use owo_colors::{OwoColorize, Rgb};

use crate::project_context::{Detected, ProjectContext};

const PKG_VERSION_COLOR: Rgb = Rgb(78, 172, 248);
const SECTION_COLOR: Rgb = Rgb(247, 104, 60);
const TOOL_COLOR: Rgb = Rgb(117, 251, 76);

pub fn write_project_context(mut writer: impl Write, context: &ProjectContext) -> io::Result<()> {
    writeln!(
        writer,
        "Uridim {}",
        env!("CARGO_PKG_VERSION").color(PKG_VERSION_COLOR)
    )?;
    writeln!(writer, "Project: {}", context.root.display())?;

    if context.vcs.is_empty()
        && context.ecosystems.is_empty()
        && context.frameworks.is_empty()
        && context.build_systems.is_empty()
        && context.infrastructure.is_empty()
    {
        writeln!(writer)?;
        writeln!(writer, "No project context found.")?;

        return Ok(());
    }

    write_section(&mut writer, "Version Control Systems", &context.vcs)?;
    write_section(&mut writer, "Ecosystems", &context.ecosystems)?;
    write_section(&mut writer, "Frameworks", &context.frameworks)?;
    write_section(&mut writer, "Build Systems", &context.build_systems)?;
    write_section(&mut writer, "Infrastructure", &context.infrastructure)?;

    Ok(())
}

fn write_section<E>(
    writer: &mut impl Write,
    title: &str,
    detected: &[Detected<E>],
) -> io::Result<()>
where
    E: Display,
{
    if detected.is_empty() {
        return Ok(());
    }

    writeln!(writer)?;
    writeln!(writer, "{}", title.color(SECTION_COLOR))?;

    for item in detected {
        writeln!(writer, "  {}", item.evidence.color(TOOL_COLOR))?;
        writeln!(writer, "  └─ {}", item.source_path.display())?;
    }

    Ok(())
}
