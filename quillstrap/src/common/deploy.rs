use crate::prelude::*;

// TODO: change file name

#[derive(PartialEq)]
pub enum UbootStuffStatus {
    NotImportant,
    BootIntoSpl,
    SkipAll,
}

pub fn clear_uboot_cli(port: String) {
    sleep_millis(100);
    for _ in 0..5 {
        send_serial_ascii(port.clone(), 0x03);
        sleep_millis(50);
    }

    for _ in 0..5 {
        send_serial_message(port.clone(), "\n\r");
        sleep_millis(50);
    }
    sleep_millis(100);
}

// String is port
pub fn enter_uboot_cli() -> Result<(String, UbootStuffStatus), ()> {
    if get_serial_devices_native().is_empty() {
        show_wait_toast("Please plug in the serial port to the pinenote and to the host");
    }

    let port = choose_serial_port();
    info!("Serial port choosed: {}", port);

    if port == "" {
        warn!(
            "No serial port selected, skipping the step of entering uboot cli mode, lets say you were already there?"
        );
        return Ok((port.clone(), SkipAll));
    }

    let message = format!(
        "Make sure U-boot serial cli is running.\nFirst reboot, then click the next button in the boot menu.\nThen use a command like \"tio -b 1500000 {}\" to enter the uboot cli.",
        port
    );
    show_wait_toast(&message);

    // Hehe
    let _ = run_command("killall -9 tio", false);
    clear_uboot_cli(port.clone());

    let version = send_read_serial(port.clone(), "version");
    if version.contains("U-Boot") {
        info!("Uboot cli confirmed, yay");
        return Ok((port.clone(), UbootStuffStatus::NotImportant));
    } else {
        error!("Failed to detect if we are in uboot cli");
        Err(())
    }
}

pub fn uboot_cli_rockusb(_options: &Options) -> Result<UbootStuffStatus, ()> {
    let (port, status) = enter_uboot_cli().unwrap();

    if status == SkipAll {
        return Ok(SkipAll);
    }
    send_serial_message(port.clone(), "rbrom\n\r");
    sleep_millis(1000);

    // TODO: confirmation from uboot about entering rbrom
    /*
    let mut serial_buf: Vec<u8> = vec![0; 200];
    read_serial(port.clone(), &mut serial_buf);
    sleep_millis(500);
    read_serial(port.clone(), &mut serial_buf);

    let ascii_chars: Vec<char> = serial_buf.iter().map(|&b| b as char).collect();
    debug!("Serial buf received: {:?}", ascii_chars);

    if ascii_chars.contains(&'/') && ascii_chars.contains(&'\\') && ascii_chars.contains(&'|') {
        info!("Detected that rockusb is on!");
    } else {
        show_wait_toast(
            "Failed to detect rockusb mode. Try to trigger it manually by typing \"rockusb 0 mmc 0\" in the serial monitor",
        );
        return Err(());
    }
    */

    show_wait_toast("Ok, now disconnect the usb dongle and connect directly to the pinenote");

    Ok(BootIntoSpl)
}

pub fn rkdevelop_test(_options: &Options) -> Result<(), ()> {
    info!("Trying rkdevelop if rockusb is connected and worked");
    let str = run_command_get_output("rkdeveloptool read-flash-info");
    debug!("Rkdevelop returned: {}", str);
    if str.contains("Did not find any rockusb device, please plug in the device") {
        return Err(());
    } else if str.contains("Flash Info:") {
        return Ok(());
    }

    Err(())
}

/*
pub fn goto_rockusb_mode(options: &Options) {
    let stat = uboot_cli_rockusb(options);
}
*/
