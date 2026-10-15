use super::Delta;

const UNITS: [&str; 5] = ["B  ", "KiB", "MiB", "GiB", "TiB"];

#[derive(Clone, Debug, Default)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub start_time: u64,
    pub user_time_millis: Delta<u64>,
    pub kernel_time_millis: Delta<u64>,
    pub pages_used: u64,
}

impl Process {
    pub fn cpu_pct(&self, time_between_readings_millis: u64) -> f32 {
        if time_between_readings_millis == 0 {
            return 0.0;
        }
        let busy_millis = self.user_time_millis.delta() + self.kernel_time_millis.delta();
        busy_millis as f32 * 100.0 / time_between_readings_millis as f32
    }

    pub fn mem_h(&self, page_size: u64) -> (f32, &'static str) {
        let mut mem_used_bytes = (self.pages_used * page_size) as f32;
        let mut unit = 0;
        
        while mem_used_bytes >= 1024.0 && unit < UNITS.len() - 1 {
            mem_used_bytes /= 1024.0;
            unit += 1;
        };

        mem_used_bytes -= mem_used_bytes % 0.1;

        (mem_used_bytes, UNITS[unit])
    }
}
