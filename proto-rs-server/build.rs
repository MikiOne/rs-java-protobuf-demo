fn main() {
    let build_enabled = option_env!("BUILD_PROTO").map(|v| v == "1").unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }
    //     prost_build::Config::new()
    //         .default_package_filename("animals")
    //         .out_dir("src/pb")
    //         .compile_protos(&["animals.proto"], &["."])
    //         .unwrap();

    prost_build::Config::new()
        // .default_package_filename("custom_type")
        .out_dir("src/pb")
        .compile_protos(&["protos/student.proto"], &["."])
        .unwrap();
}