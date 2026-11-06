//! Windows only: embed the app icon and version info (VERSIONINFO) into `evovector.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `EVOVECTOR_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/evovector.ico");
    println!("cargo:rerun-if-env-changed=EVOVECTOR_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/evovector.ico")
        .set("ProductName", "EvoVector")
        .set("FileDescription", "EvoVector vector illustration editor")
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) the EvoVector authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "evovector.exe")
        .set("InternalName", "evovector");
    if let Err(e) = res.compile() {
        if std::env::var_os("EVOVECTOR_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=evovector.exe built without icon/version resources: {e}");
    }
    Ok(())
}
