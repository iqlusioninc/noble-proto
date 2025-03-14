//! Build Noble protocol proto files. This build script clones the Noble modules
//! specified in the submodules and then uses them to build the required
//! proto files for further compilation.

use {
    regex::Regex,
    std::{
        env,
        ffi::{OsStr, OsString},
        fs::{self, create_dir_all, remove_dir_all},
        io,
        path::{Path, PathBuf},
        process,
        sync::atomic::{self, AtomicBool},
    },
    walkdir::WalkDir,
};

// Suppress log messages
static QUIET: AtomicBool = AtomicBool::new(false);

// Module directory mappings
const MODULE_MAPPINGS: &[(&str, &str, &str)] = &[
    ("noble-aura-proto", "aura", "../aura/"),
    ("noble-dollar-proto", "dollar", "../dollar/"),
    ("noble-fiattokenfactory-proto", "fiattokenfactory", "../fiattokenfactory/"),
    ("noble-forwarding-proto", "forwarding", "../forwarding/"),
    ("noble-halo-proto", "halo", "../halo/"),
    ("noble-cctp-proto", "cctp", "../noble-cctp/"),
];

// Temporary build directory
const TMP_BUILD_DIR: &str = "/tmp/noble-proto/";

// Protos belonging to these Protobuf packages will be excluded
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];

/// Log info to the console (if `QUIET` is disabled)
macro_rules! info {
    ($msg:expr) => {
        if !is_quiet() {
            println!("[info] {}", $msg)
        }
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    fs::create_dir_all(&tmp_build_dir).unwrap();

    update_submodules();

    // Process each module
    for &(crate_name, proto_name, module_dir) in MODULE_MAPPINGS {
        let proto_dir = Path::new(module_dir).join("proto");
        let out_tmp_dir = tmp_build_dir.join(proto_name);
        let final_out_dir = format!("../../{}/src/prost", crate_name);
        
        fs::create_dir_all(&out_tmp_dir).unwrap();
        
        info!("Compiling protos for {} module", proto_name);
        
        compile_protos_and_services(&proto_dir, &out_tmp_dir);
        
        let final_out_path = PathBuf::from(&final_out_dir);
        copy_generated_files(&out_tmp_dir, &final_out_path);
        
        run_rustfmt(&final_out_path);
    }

    if is_github() {
        println!("Rebuilt Noble protocol protos");
    }
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = 
        process::Command::new(&cmd).args(args).stdout(stdout).status().unwrap_or_else(|e| match e
            .kind()
        {
            io::ErrorKind::NotFound => {
                panic!("error running '{:?}': command not found. Is it installed?", cmd.as_ref())
            },
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_cmd("git", args)
}

fn run_rustfmt(dir: &Path) {
    info!("Running rustfmt on prost/tonic-generated code");

    let mut args = ["--edition", "2021"].iter().map(Into::into).collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn update_submodules() {
    info!("Updating Noble protocol submodules...");
    run_git(["submodule", "update", "--init", "--recursive"]);
}

fn compile_protos_and_services(proto_path: &Path, out_dir: &Path) {
    info!(
        "Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);

    // Find all proto files
    let mut protos: Vec<PathBuf> = vec![];
    protos.append(
        &mut WalkDir::new(proto_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().is_some()
                    && e.path().extension().unwrap() == "proto"
            })
            .map(|e| e.into_path())
            .collect(),
    );

    // Use prost_build to compile the protos
    if !protos.is_empty() {
        let proto_include_paths: Vec<PathBuf> = vec![proto_path.to_path_buf()];
        
        tonic_build::configure()
            .build_client(true)
            .build_server(false)
            .format(true)
            .out_dir(out_dir)
            .compile_with_config(config, &protos, &proto_include_paths)
            .unwrap();
    }

    info!("=> Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Use cosmos_sdk_proto for cosmos types 
        ("(super::)+cosmos", "cosmos_sdk_proto::cosmos"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
    ];

    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &contents)
}