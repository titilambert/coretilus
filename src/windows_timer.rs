#[cfg(target_os = "windows")]
use windows::Win32::Media::timeBeginPeriod;

/// Boost the Windows timer resolution to 1 ms.
/// On non‑Windows targets this is a no‑op.
pub fn boost_timer_resolution() {
    #[cfg(target_os = "windows")]
    unsafe {
        // According to the Win32 API, timeBeginPeriod(1) requests a
        // timer granularity of 1 millisecond. The function returns a
        // count of outstanding requests; we ignore it here because we only
        // ever call it once at startup.
        let _ = timeBeginPeriod(1u32);
    }
}
