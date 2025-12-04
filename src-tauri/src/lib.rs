#[cfg(target_os = "linux")]
pub fn show_notification(title: &str, body: &str) {
    use std::process::Command;
    
    // Use notify-send if available (most common on Linux)
    let _ = Command::new("notify-send")
        .arg(title)
        .arg(body)
        .arg("-u").arg("normal")
        .arg("-t").arg("5000") // 5 seconds timeout
        .spawn();
}

#[cfg(not(target_os = "linux"))]
pub fn show_notification(_title: &str, _body: &str) {
    // Placeholder for other OSes
}
