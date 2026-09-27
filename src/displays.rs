//! Physical virtual-desktop coordinates, independent of the overlay resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Display {
    pub id: String,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub primary: bool,
}
impl Display {
    pub fn width(&self) -> i64 {
        i64::from(self.right) - i64::from(self.left)
    }
    pub fn height(&self) -> i64 {
        i64::from(self.bottom) - i64::from(self.top)
    }
    pub fn project(&self, x: i32, y: i32) -> Option<(f64, f64)> {
        if self.width() <= 0
            || self.height() <= 0
            || x < self.left
            || x >= self.right
            || y < self.top
            || y >= self.bottom
        {
            return None;
        }
        Some((
            (i64::from(x) - i64::from(self.left)) as f64 / self.width() as f64,
            (i64::from(y) - i64::from(self.top)) as f64 / self.height() as f64,
        ))
    }
}
/// None means the initial default (primary); an absent explicit ID never falls back.
pub fn selected<'a>(displays: &'a [Display], id: Option<&str>) -> Option<&'a Display> {
    match id {
        Some(id) => displays.iter().find(|d| d.id == id),
        None => displays.iter().find(|d| d.primary),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn display(id: &str, left: i32, top: i32, width: i32, height: i32, primary: bool) -> Display {
        Display {
            id: id.into(),
            left,
            top,
            right: left + width,
            bottom: top + height,
            primary,
        }
    }
    #[test]
    fn secondary_on_left_above_and_portrait() {
        for d in [
            display("left", -2560, 0, 2560, 1440, false),
            display("above", 0, -2160, 3840, 2160, false),
            display("portrait", 1920, -400, 1080, 1920, false),
        ] {
            assert_eq!(
                d.project(
                    d.left + (d.width() / 2) as i32,
                    d.top + (d.height() / 2) as i32
                ),
                Some((0.5, 0.5))
            );
            assert_eq!(d.project(d.left, d.top), Some((0.0, 0.0)));
            assert_eq!(d.project(d.right, d.top), None);
            assert_eq!(d.project(d.left, d.bottom), None);
        }
    }
    #[test]
    fn shared_edge_belongs_to_only_one_monitor() {
        let a = display("1", 0, 0, 1920, 1080, true);
        let b = display("2", 1920, 0, 2560, 1440, false);
        assert_eq!(a.project(1920, 0), None);
        assert_eq!(b.project(1920, 0), Some((0.0, 0.0)));
        assert_eq!(b.project(100, 100), None);
    }
    #[test]
    fn selection_survives_reorder_and_never_falls_back_on_disconnect() {
        let a = display("1", 0, 0, 1920, 1080, true);
        let b = display("2", -2560, 0, 2560, 1440, false);
        let c = display("3", 0, -2160, 3840, 2160, false);
        assert_eq!(
            selected(&[c.clone(), b.clone(), a.clone()], Some("2")),
            Some(&b)
        );
        assert_eq!(selected(&[a.clone(), c.clone()], Some("2")), None);
        assert_eq!(selected(&[c, b, a.clone()], None), Some(&a));
    }
    #[test]
    fn resizing_updates_mapping_without_dpi_or_obs_scaling_assumptions() {
        let a = display("2", 1920, 0, 3840, 2160, false);
        let b = display("2", 1920, 0, 1920, 1080, false);
        assert_eq!(a.project(2880, 540), Some((0.25, 0.25)));
        assert_eq!(b.project(2880, 540), Some((0.5, 0.5)));
        assert_eq!(display("invalid", 0, 0, 0, 0, false).project(0, 0), None);
    }
}
