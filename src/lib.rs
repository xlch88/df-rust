use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use sysinfo::{Disk, Disks};

#[napi(object)]
pub struct FsStat {
    pub fstype: String,
    pub source: String,
    pub size: BigInt,
    pub used: BigInt,
    pub avail: BigInt,
    pub pcent: f64,
    pub target: String,
}

#[napi]
pub fn df() -> Vec<FsStat> {
    let mut disks = Disks::new_with_refreshed_list();
    disks.refresh();

    disks
        .list()
        .iter()
        .map(|d: &Disk| {
            let total = d.total_space();
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pcent = if total == 0 {
                0.0
            } else {
                ((used as f64) * 100.0 / (total as f64) * 1000.0).round() / 1000.0
            };

            FsStat {
                fstype: d.file_system().to_string_lossy().into_owned(),
                source: d.name().to_string_lossy().into_owned(),
                size: BigInt::from(total),
                used: BigInt::from(used),
                avail: BigInt::from(avail),
                pcent,
                target: d.mount_point().to_string_lossy().into_owned(),
            }
        })
        .collect()
}
