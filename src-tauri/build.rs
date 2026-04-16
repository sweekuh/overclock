fn main() {
    // Only require admin elevation in release builds (production .exe)
    // Dev builds run without elevation so `tauri dev` works normally
    #[cfg(not(debug_assertions))]
    let windows = tauri_build::WindowsAttributes::new()
        .app_manifest(r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
                <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                    <security>
                        <requestedPrivileges>
                            <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
                        </requestedPrivileges>
                    </security>
                </trustInfo>
            </assembly>
        "#);

    #[cfg(debug_assertions)]
    let windows = tauri_build::WindowsAttributes::new();

    let attrs = tauri_build::Attributes::new()
        .windows_attributes(windows);

    tauri_build::try_build(attrs)
        .expect("failed to run tauri build");
}
