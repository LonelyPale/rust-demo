pub fn version() {
    //请注意，这个方法只能获取到Rust编译时所运行的操作系统类型，而不能动态获取运行时操作系统信息。
    //如果你需要更详细的操作系统信息，如版本号或内核信息，你可能需要使用第三方库来进行系统调用或查询系统信息。
    let os_type = std::env::consts::OS; //linux、macos、windows、ios、android
    println!("编译时-操作系统类型: {}", os_type);

    //在编译时评估配置标志的布尔组合。
    if cfg!(target_os = "windows") {
        println!("Hello Windows");
    } else if cfg!(target_os = "linux") {
        println!("Hello Linux");
    } else if cfg!(target_os = "macos") {
        println!("Hello MacOS");
    } else {
        println!("Unknown os");
    }
}

#[test]
fn test() {
    version()
}
