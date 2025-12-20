fn main() {
    let occt_root = "C:\\OCCT\\opencascade-7.9.3-vc14-64";
    let occt_include = format!("{}\\inc", occt_root);
    let occt_lib = format!("{}\\win64\\vc14\\lib", occt_root);

    println!("cargo:rustc-link-search=native={}", occt_lib);

    // Core TKs for basic CAD
    println!("cargo:rustc-link-lib=TKernel");
    println!("cargo:rustc-link-lib=TKMath");
    println!("cargo:rustc-link-lib=TKG2d");
    println!("cargo:rustc-link-lib=TKG3d");
    println!("cargo:rustc-link-lib=TKBRep");
    println!("cargo:rustc-link-lib=TKGeomBase");
    println!("cargo:rustc-link-lib=TKGeomAlgo");
    println!("cargo:rustc-link-lib=TKTopAlgo");
    println!("cargo:rustc-link-lib=TKPrim");
    println!("cargo:rustc-link-lib=TKBO");
    println!("cargo:rustc-link-lib=TKBool");

    #[cfg(feature = "occt")]
    {
        cxx_build::bridge("src/cad.rs")
            .file("src/bridge.cpp")
            .include(&occt_include)
            .flag_if_supported("/std:c++17")
            .compile("oasm-occt-bridge");

        println!("cargo:rerun-if-changed=src/cad.rs");
        println!("cargo:rerun-if-changed=src/bridge.cpp");
    }
}
