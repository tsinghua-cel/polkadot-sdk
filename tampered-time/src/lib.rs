use core::time::Duration;
use std::time::SystemTime;
use std::sync::OnceLock;
use std::env;

// 环境变量名称：设置时间偏移（秒），允许为负数。例如：TAMPERED_TIME_OFFSET=-30
// 如果未设置或解析失败，则回退为 10。
const ENV_VAR: &str = "TAMPERED_TIME_OFFSET";
const FALLBACK_OFFSET: i64 = 10;

static OFFSET: OnceLock<i64> = OnceLock::new();

fn current_offset() -> i64 {
    *OFFSET.get_or_init(|| {
        match env::var(ENV_VAR) {
            Ok(v) => v.trim().parse::<i64>().unwrap_or(FALLBACK_OFFSET),
            Err(_) => FALLBACK_OFFSET,
        }
    })
}

/// Returns current duration since unix epoch.
pub fn duration_now() -> Duration {
    let now = SystemTime::now();
    let real = now.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_else(|e| {
        panic!("Current time {:?} is before unix epoch. Something is wrong: {:?}", now, e)
    });
    apply_offset(real, current_offset())
}

/// Returns current system time with offset applied.
pub fn now() -> SystemTime {
    SystemTime::UNIX_EPOCH + duration_now()
}

fn apply_offset(real: Duration, offset: i64) -> Duration {
    let secs = real.as_secs();
    let nanos = real.subsec_nanos();
    if offset == 0 { return real; }
    if offset > 0 {
        let off = offset as u64;
        match secs.checked_add(off) {
            Some(v) => Duration::from_secs(v) + Duration::from_nanos(nanos as u64),
            None => Duration::from_secs(u64::MAX), 
        }
    } else {
        let off = (-offset) as u64;
        if off >= secs { Duration::from_secs(0) } else { Duration::from_secs(secs - off) + Duration::from_nanos(nanos as u64) }
    }
}
