//! Names/properties come from udev; event numbers are never persisted as identity.
use std::path::PathBuf;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Info {
    pub id: String,
    pub path: PathBuf,
    pub name: String,
    pub keyboard: bool,
    pub mouse: bool,
    pub serial: Option<String>,
    pub physical: Option<String>,
}
pub fn info(device: &udev::Device) -> Option<Info> {
    let path = device.devnode()?.to_path_buf();
    let name = path.file_name()?.to_str()?;
    if path.parent()? != std::path::Path::new("/dev/input")
        || !name
            .strip_prefix("event")?
            .bytes()
            .all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let flag = |key| device.property_value(key).is_some_and(|v| v == "1");
    let keyboard = flag("ID_INPUT_KEYBOARD");
    let mouse = flag("ID_INPUT_MOUSE") || flag("ID_INPUT_TOUCHPAD");
    if !keyboard && !mouse {
        return None;
    }
    let parent = device.parent()?;
    let text =
        |v: Option<&std::ffi::OsStr>| v.map(|s| s.to_string_lossy().trim_matches('"').to_owned());
    Some(Info {
        id: device.syspath().to_string_lossy().into_owned(),
        path,
        name: text(parent.attribute_value("name")).unwrap_or_else(|| "Input device".into()),
        keyboard,
        mouse,
        serial: text(device.property_value("ID_SERIAL")),
        physical: text(device.property_value("ID_PATH")),
    })
}
pub fn enumerate() -> anyhow::Result<Vec<(udev::Device, Info)>> {
    let mut e = udev::Enumerator::new()?;
    e.match_subsystem("input")?;
    Ok(e.scan_devices()?
        .filter_map(|d| info(&d).map(|i| (d, i)))
        .collect())
}
