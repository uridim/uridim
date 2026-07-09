use std::path::PathBuf;

use owo_colors::{OwoColorize, Rgb};

use crate::candidate::{
    BuildSystemEvidence, EcosystemEvidence, FrameworkEvidence, InfrastructureEvidence, VcsEvidence,
};
use crate::project_context::{Detected, ProjectContext};
use crate::report::write_project_context;

const PKG_VERSION_COLOR: Rgb = Rgb(78, 172, 248);
const SECTION_COLOR: Rgb = Rgb(247, 104, 60);
const TOOL_COLOR: Rgb = Rgb(117, 251, 76);

fn empty_context() -> ProjectContext {
    ProjectContext {
        root: PathBuf::from("/project"),
        vcs: Vec::new(),
        ecosystems: Vec::new(),
        frameworks: Vec::new(),
        build_systems: Vec::new(),
        infrastructure: Vec::new(),
    }
}

fn detected<E>(evidence: E, source_path: &str) -> Detected<E> {
    Detected {
        evidence,
        scope_path: PathBuf::from("/project"),
        source_path: PathBuf::from(source_path),
    }
}

fn render(context: &ProjectContext) -> String {
    let mut output = Vec::new();

    write_project_context(&mut output, context).unwrap();

    String::from_utf8(output).unwrap()
}

#[test]
fn reports_when_no_project_context_is_found() {
    let context = empty_context();

    let output = render(&context);

    let expected = format!(
        "Uridim {}\nProject: /project\n\nNo project context found.\n",
        env!("CARGO_PKG_VERSION").color(PKG_VERSION_COLOR)
    );

    assert_eq!(output, expected);
}

#[test]
fn writes_complete_project_context_in_deterministic_order() {
    let context = ProjectContext {
        root: PathBuf::from("/project"),
        vcs: vec![detected(VcsEvidence::Git, "/project/.git")],
        ecosystems: vec![detected(EcosystemEvidence::Cargo, "/project/Cargo.toml")],
        frameworks: vec![detected(FrameworkEvidence::NextJs, "/project/package.json")],
        build_systems: vec![detected(
            BuildSystemEvidence::CMake,
            "/project/CMakeLists.txt",
        )],
        infrastructure: vec![detected(
            InfrastructureEvidence::DockerCompose,
            "/project/compose.yaml",
        )],
    };

    let output = render(&context);

    let expected = format!(
        concat!(
            "Uridim {}\n",
            "Project: /project\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/.git\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/Cargo.toml\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/package.json\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/CMakeLists.txt\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/compose.yaml\n",
        ),
        env!("CARGO_PKG_VERSION").color(PKG_VERSION_COLOR),
        "Version Control Systems".color(SECTION_COLOR),
        VcsEvidence::Git.color(TOOL_COLOR),
        "Ecosystems".color(SECTION_COLOR),
        EcosystemEvidence::Cargo.color(TOOL_COLOR),
        "Frameworks".color(SECTION_COLOR),
        FrameworkEvidence::NextJs.color(TOOL_COLOR),
        "Build Systems".color(SECTION_COLOR),
        BuildSystemEvidence::CMake.color(TOOL_COLOR),
        "Infrastructure".color(SECTION_COLOR),
        InfrastructureEvidence::DockerCompose.color(TOOL_COLOR),
    );

    assert_eq!(output, expected);
}

#[test]
fn omits_empty_sections() {
    let context = ProjectContext {
        root: PathBuf::from("/project"),
        vcs: Vec::new(),
        ecosystems: vec![detected(EcosystemEvidence::Cargo, "/project/Cargo.toml")],
        frameworks: Vec::new(),
        build_systems: Vec::new(),
        infrastructure: vec![detected(
            InfrastructureEvidence::DockerCompose,
            "/project/compose.yaml",
        )],
    };

    let output = render(&context);

    let expected = format!(
        concat!(
            "Uridim {}\n",
            "Project: /project\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/Cargo.toml\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/compose.yaml\n",
        ),
        env!("CARGO_PKG_VERSION").color(PKG_VERSION_COLOR),
        "Ecosystems".color(SECTION_COLOR),
        EcosystemEvidence::Cargo.color(TOOL_COLOR),
        "Infrastructure".color(SECTION_COLOR),
        InfrastructureEvidence::DockerCompose.color(TOOL_COLOR),
    );

    assert_eq!(output, expected);
}

#[test]
fn preserves_detection_order_within_a_section() {
    let context = ProjectContext {
        root: PathBuf::from("/project"),
        vcs: Vec::new(),
        ecosystems: vec![
            detected(EcosystemEvidence::Cargo, "/project/Cargo.toml"),
            detected(EcosystemEvidence::NodeJs, "/project/package.json"),
        ],
        frameworks: Vec::new(),
        build_systems: Vec::new(),
        infrastructure: Vec::new(),
    };

    let output = render(&context);

    let expected = format!(
        concat!(
            "Uridim {}\n",
            "Project: /project\n",
            "\n",
            "{}\n",
            "  {}\n",
            "  └─ /project/Cargo.toml\n",
            "  {}\n",
            "  └─ /project/package.json\n",
        ),
        env!("CARGO_PKG_VERSION").color(PKG_VERSION_COLOR),
        "Ecosystems".color(SECTION_COLOR),
        EcosystemEvidence::Cargo.color(TOOL_COLOR),
        EcosystemEvidence::NodeJs.color(TOOL_COLOR),
    );

    assert_eq!(output, expected);
}
