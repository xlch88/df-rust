use napi::bindgen_prelude::BigInt; // 有了 napi6 才找得到:contentReference[oaicite:5]{index=5}
use napi_derive::napi;
use sysinfo::{Disk, Disks}; // 全新的磁盘接口:contentReference[oaicite:6]{index=6}

const MIB: u64 = 1024 * 1024;

#[napi(object)]
pub struct FsStat {
    pub fstype: String,
    pub source: String,
    pub size: BigInt, // 直接丢 BigInt，JS 里就是 bigint
    pub used: BigInt,
    pub avail: BigInt,
    pub pcent: String,
    pub target: String,
}

#[napi]
pub fn df() -> Vec<FsStat> {
    // 新建并立即刷新列表
    let mut disks = Disks::new_with_refreshed_list(); // 创建时自动扫盘:contentReference[oaicite:7]{index=7}
    disks.refresh(); // 再拉一次实时数据:contentReference[oaicite:8]{index=8}

    disks
        .list()
        .iter()
        .map(|d: &Disk| {
            let total = d.total_space() / MIB;
            let avail = d.available_space() / MIB;
            let used = total.saturating_sub(avail);
            let pcent = if total == 0 {
                "0%".to_string()
            } else {
                format!("{}%", used * 100 / total)
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
