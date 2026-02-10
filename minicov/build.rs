use cc::Build;
use walkdir::WalkDir;

fn main() {
    let mut cfg = Build::new();
    cfg.compiler("clang");
    cfg.flag("-nostdlibinc");
    cfg.flag("-fno-stack-protector");
    cfg.flag("-fno-profile-generate");
    cfg.flag("-fno-profile-use");
    cfg.flag("-fno-profile-instr-generate");
    cfg.flag("-fno-profile-instr-use");
    cfg.flag("-fno-coverage-mapping");
    cfg.warnings(false);
    cfg.define("COMPILER_RT_HAS_ATOMICS", "1");
    cfg.define("COMPILER_RT_PROFILE_BAREMETAL", "1");

    if let Ok(include) = std::env::var("INCLUDE") {
        for path in include.split(';') {
            cfg.include(path);
        }
    }

    let sources = vec![
        "c/InstrProfiling.c",
        "c/InstrProfilingBuffer.c",
        "c/InstrProfilingInternal.c",
        "c/InstrProfilingMerge.c",
        "c/InstrProfilingMergeFile.c",
        "c/InstrProfilingPlatformLinux.c",
        "c/InstrProfilingPlatformWindows.c",
        "c/InstrProfilingWriter.c",
        "c/InstrProfilingValue.c",
        "c/InstrProfilingVersionVar.c",
    ];

    for source in &sources {
        cfg.file(source);
    }

    cfg.compile("llvm_profiler_runtime");

    for entry in WalkDir::new("c") {
        println!(
            "cargo:rerun-if-changed={}",
            entry.unwrap().path().to_str().unwrap()
        );
    }
}
