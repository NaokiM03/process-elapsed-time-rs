use std::{fs, io::Result};

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_long = i64;
#[allow(non_camel_case_types)]
type c_ulonglong = u64;

const _SC_CLK_TCK: c_int = 2;

extern "C" {
    fn sysconf(name: c_int) -> c_long;
}

fn starttime() -> Result<u64> {
    // awk '{print $22}' /proc/self/stat
    let starttime = fs::read_to_string("/proc/self/stat")?
        .split_ascii_whitespace()
        .nth(22 - 1)
        .expect("Failed to get starttime")
        .parse::<c_ulonglong>()
        .expect("Failed to parse starttime");

    let clock_ticks = unsafe { sysconf(_SC_CLK_TCK) as u64 };

    let starttime = starttime / clock_ticks;
    Ok(starttime)
}

fn uptime() -> Result<u64> {
    // awk '{print $1}' /proc/uptime
    let uptime = fs::read_to_string("/proc/uptime")?
        .split_ascii_whitespace()
        .nth(0)
        .expect("Failed to get uptime")
        .parse::<f64>()
        .expect("Failed to parse uptime")
        .floor() as u64;

    Ok(uptime)
}

pub fn elapsed_time() -> Result<u64> {
    let elapsed_time = uptime()? - starttime()?;
    Ok(elapsed_time)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_starttime() {
//         let result = starttime().unwrap();

//         let expect = {
//             let output = std::process::Command::new("awk")
//                 .args(["{print $22}", "/proc/self/stat"])
//                 .output()
//                 .unwrap();
//             let stdout = output.stdout;
//             let starttime = stdout
//                 .iter()
//                 .filter(|&&x| x != '\n' as u8)
//                 .map(|&x| x as char)
//                 .collect::<String>();

//             let clock_ticks = unsafe { sysconf(_SC_CLK_TCK) as u64 };

//             starttime.parse::<u64>().unwrap() / clock_ticks
//         };

//         assert_eq!(result, expect);
//     }

//     #[test]
//     fn test_uptime() {
//         let result = uptime().unwrap();

//         let expect = {
//             let output = std::process::Command::new("awk")
//                 .args(["{print $1}", "/proc/uptime"])
//                 .output()
//                 .unwrap();
//             let stdout = output.stdout;
//             let starttime = stdout
//                 .iter()
//                 .filter(|&&x| x != '\n' as u8)
//                 .map(|&x| x as char)
//                 .collect::<String>();

//             starttime.parse::<f64>().unwrap().floor() as u64
//         };

//         assert_eq!(result, expect);
//     }

//     #[test]
//     fn test_elapsed_time() {
//         let time_secs = 3;

//         std::thread::sleep(std::time::Duration::from_secs(time_secs));
//         let result = elapsed_time().unwrap();

//         assert_eq!(result, time_secs);
//     }
// }
