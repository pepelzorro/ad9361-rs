//! Build

#[macro_use]
extern crate error_chain;
error_chain! {
    foreign_links {
        Io(::std::io::Error);
        EnvVar(::std::env::VarError);
        StringFromUtf8(::std::string::FromUtf8Error);
    }
}

macro_rules! truthy_cfg {
    ($($cfg : tt) *) => {
        if cfg!($($cfg) *) {
            Some("1")
        } else {
            Some("0")
        }
    };
}

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn no_os() -> PathBuf {
    manifest_dir().join("no-os")
}

fn is_cross_compiling() -> Result<bool> {
    Ok(env::var("TARGET")? != env::var("HOST")?)
}

fn get_command_result(command: &mut Command) -> Result<String> {
    command
        .output()
        .chain_err(|| "Couldn't find target GCC executable.")
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout)?)
            } else {
                panic!("Couldn't read output from GCC.")
            }
        })
}

trait CompilationBuilder {
    fn flag(&mut self, s: &str) -> &mut Self;
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self;

    /// Build flags for ad9361 sources
    fn ad9361_build_setup(&mut self) -> &mut Self {
        let target = env::var("TARGET").unwrap_or_else(|_| "".to_string());

        let build = self
            .flag("-fno-exceptions")
            .flag("-fno-unwind-tables")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-fno-delete-null-pointer-checks")
            // use a full word for enums, this should match clang's behaviour
            .flag("-fno-short-enums")
            .define("AXI_ADC_NOT_PRESENT", None);

        // print errors and warnings, use log framework to filter them
        let build = build.define("HAVE_VERBOSE_MESSAGES", None);
        let build = if cfg!(feature = "debug_messages") {
            build.define("HAVE_DEBUG_MESSAGES", None)
        } else {
            build
        };

        // split tables, increases code size
        let build = build
            .define("HAVE_SPLIT_GAIN_TABLE", Some("1"))
            .define("HAVE_TDD_SYNTH_TABLE", Some("1"));

        // device flag selection
        let build = build
            .define("AD9361_DEVICE", truthy_cfg!(feature = "ad9361_device"))
            .define("AD9364_DEVICE", truthy_cfg!(feature = "ad9364_device"))
            .define("AD9363A_DEVICE", truthy_cfg!(feature = "ad9363a_device"));

        // warnings on by default
        build
            .flag("-Wvla")
            .flag("-Wall")
            .flag("-Wextra")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-write-strings")
            .flag("-Wno-sign-compare");

        if target.starts_with("thumb") {
            // unaligned accesses are usually a poor idea on ARM cortex-m
            build.flag("-mno-unaligned-access")
        } else {
            build
        }
    }
}
impl CompilationBuilder for cpp_build::Config {
    fn flag(&mut self, s: &str) -> &mut Self {
        self.flag(s)
    }
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self {
        self.define(var, val)
    }
}
impl CompilationBuilder for cc::Build {
    fn flag(&mut self, s: &str) -> &mut Self {
        self.flag(s)
    }
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self {
        self.define(var, val)
    }
}

fn cc_ad9361_library() {
    let no_os = no_os();
    let ad9361 = no_os.join("drivers/rf-transceiver/ad9361");
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ad9361_lib_name = Path::new(&out_dir).join("libad9361.a");

    if is_cross_compiling().unwrap() {
        // Find include directory used by the crosscompiler for libm
        let mut gcc = cc::Build::new().get_compiler().to_command();
        let libm_location = PathBuf::from(
            get_command_result(gcc.arg("--print-file-name=libm.a"))
                .expect("Error querying gcc for libm location"),
        );
        let libm_path = libm_location.parent().unwrap();

        // Pass this to the linker
        println!(
            "cargo:rustc-link-search=native={}",
            libm_path.to_string_lossy()
        );
        println!("cargo:rustc-link-lib=static=m");
    }

    println!("Building ad9361..");
    let _target = env::var("TARGET").unwrap_or_else(|_| "".to_string());
    let start = Instant::now();

    let mut builder = cc::Build::new();
    let builder_ref = builder
        .ad9361_build_setup()
        .define("malloc", Some("admalloc"))
        .define("calloc", Some("adcalloc"))
        .define("free", Some("adfree"))
        .opt_level(0)
        // include
        .include(ad9361.join(""))
        .include(no_os.join("include"))
        .include("./csrc")
        // files
        .file(ad9361.join("ad9361.c"))
        .file(ad9361.join("ad9361_api.c"))
        .file(ad9361.join("ad9361_util.c"))
        .file("./csrc/micro_string.cc");

    // Compile
    builder_ref.compile("ad9361");

    println!("Building ad9361 from source took {:?}", start.elapsed());
}

/// Configure bindgen for cross-compiling
fn bindgen_cross_builder() -> Result<bindgen::Builder> {
    let builder = bindgen::Builder::default().clang_arg("--verbose");

    if is_cross_compiling()? {
        // Setup target triple
        let target = env::var("TARGET")?;
        let builder = builder.clang_arg(format!("--target={}", target));
        println!("Setting bindgen to cross compile to {}", target);

        // Find the sysroot used by the crosscompiler, and pass this to clang
        let mut gcc = cc::Build::new().get_compiler().to_command();
        let path = get_command_result(gcc.arg("--print-sysroot"))?;
        let builder = builder.clang_arg(format!("--sysroot={}", path.trim()));

        Ok(builder)
    } else {
        Ok(builder)
    }
}

/// This generates "ad9361_types.rs" containing structs and enums which are
/// interoperable with rust
fn bindgen_ad9361() {
    use bindgen::*;

    let out_dir = env::var("OUT_DIR").unwrap();
    let _ad9361_types_name = Path::new(&out_dir).join("ad9361_types.rs");

    println!("Running bindgen");
    let start = Instant::now();

    let bindings = bindgen_cross_builder()
        .expect("Error setting up bindgen for cross compiling")
        .allowlist_recursively(true)
        .prepend_enum_name(false)
        .impl_debug(true)
        .layout_tests(true)
        .derive_default(true)
        .size_t_is_usize(true)
        .use_core()
        .ctypes_prefix("cty")
        // Types - blocklist
        .blocklist_type("std")
        .blocklist_type("_Float64x")
        // Functions - blocklist for using u128 (no stable rust ABI)
        .blocklist_function("strtold")
        .blocklist_function("strtold_l")
        .blocklist_function("strfroml")
        .blocklist_function("strtof64x")
        .blocklist_function("strtof64x_l")
        .blocklist_function("strfromf64x")
        .blocklist_function("qecvt")
        .blocklist_function("qfcvt")
        .blocklist_function("qgcvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt_r")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_partialeq(true)
        .derive_eq(true)
        .header("csrc/ad9361_wrapper.h")
        .clang_arg("-I./no-os/drivers/rf-transceiver/ad9361/")
        .clang_arg("-I./no-os/include")
        .clang_arg("-I./csrc")
        .clang_arg("-DAXI_ADC_NOT_PRESENT");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to $OUT_DIR/ad9361_types.rs
    let out_path = PathBuf::from(out_dir).join("ad9361_types.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    println!("Running bindgen took {:?}", start.elapsed());
}

fn build_inline_cpp() {
    let no_os = no_os();
    let ad9361 = no_os.join("drivers/rf-transceiver/ad9361");

    println!("Building inline cpp");
    let start = Instant::now();

    cpp_build::Config::new()
        .include(ad9361)
        .include(no_os.join("include"))
        .include("csrc")
        .ad9361_build_setup()
        .cpp_link_stdlib(None)
        .flag("-std=c++14")
        .build("src/lib.rs");

    println!("Building inline cpp took {:?}", start.elapsed());
}

fn main() {
    bindgen_ad9361();
    build_inline_cpp();
    cc_ad9361_library();
}
