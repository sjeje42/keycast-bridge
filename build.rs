use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=data/icons");
    println!("cargo:rerun-if-changed=data/icons.gresource.xml");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=WINDRES");
    if env::var_os("CARGO_FEATURE_GUI").is_none() {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let status = Command::new("glib-compile-resources")
        .arg("data/icons.gresource.xml")
        .arg("--sourcedir=data")
        .arg("--target")
        .arg(out.join("icons.gresource"))
        .status()
        .expect("Install GLib development tools (glib-compile-resources)");
    assert!(status.success(), "Could not compile application icons");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    // The supported Windows build uses the MSYS2 UCRT64 GNU toolchain.
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").unwrap(), "gnu");
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let numeric = ["MAJOR", "MINOR", "PATCH"]
        .map(|part| env::var(format!("CARGO_PKG_VERSION_{part}")).unwrap())
        .join(",");
    let flags = if env::var("CARGO_PKG_VERSION_PRE").unwrap().is_empty() {
        0
    } else {
        2 // VS_FF_PRERELEASE
    };
    let icon = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("data/icons/keycast-bridge.ico")
        .to_string_lossy()
        .replace('\\', "/");
    let rc = out.join("keycast-bridge.rc");
    fs::write(
        &rc,
        format!(
            r#"1 ICON "{icon}"
1 VERSIONINFO
FILEVERSION {numeric},0
PRODUCTVERSION {numeric},0
FILEFLAGSMASK 0x3fL
FILEFLAGS {flags}
FILEOS 0x40004L
FILETYPE 0x1L
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904b0"
    BEGIN
      VALUE "CompanyName", "Keycast Bridge contributors\0"
      VALUE "FileDescription", "Keycast Bridge - shortcut overlay for OBS\0"
      VALUE "FileVersion", "{version}\0"
      VALUE "InternalName", "keycast-bridge\0"
      VALUE "OriginalFilename", "keycast-bridge.exe\0"
      VALUE "ProductName", "Keycast Bridge\0"
      VALUE "ProductVersion", "{version}\0"
      VALUE "LegalCopyright", "Keycast Bridge contributors - GPL-3.0-only\0"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x0409, 1200
  END
END
"#
        ),
    )
    .unwrap();
    let object = out.join("keycast-bridge-res.o");
    let compiler = env::var("WINDRES").unwrap_or_else(|_| "windres".into());
    let status = Command::new(compiler)
        .args(["--input-format=rc", "--output-format=coff", "--codepage=65001"])
        .arg(&rc)
        .arg(&object)
        .status()
        .expect("Install MinGW binutils (windres)");
    assert!(status.success(), "Could not compile Windows icon and metadata");
    println!("cargo:rustc-link-arg-bin=keycast-bridge={}", object.display());
}
