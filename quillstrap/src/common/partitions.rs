use std::process::Command;

use crate::prelude::*;

const BLOCK_SIZE: usize = 512;

pub fn choose_disk() -> String {
    let output = Command::new("lsblk")
        .arg("-ndo")
        .arg("NAME,TYPE")
        .output()
        .expect("Failed to run lsblk");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let disks: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 && parts[1] == "disk" {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect();

    if disks.is_empty() {
        println!("No disks found!");
        return "".to_string();
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a disk")
        .default(0)
        .items(&disks)
        .interact()
        .expect("Failed to get user selection for disk");

    format!("/dev/{}", disks[selection].clone())
}

pub fn get_disk_partitions(disk: &str) -> Vec<String> {
    let mut partitions = Vec::new();

    let output = Command::new("lsblk")
        .arg("-ln")
        .arg("-o")
        .arg("NAME")
        .arg(disk)
        .output()
        .expect(&format!("Failed to run lsblk for disk: {}", disk));

    if !output.status.success() {
        return partitions;
    }

    let lines = String::from_utf8_lossy(&output.stdout);
    for line in lines.lines().skip(1) {
        // Skip the disk itself
        let name = line.trim();
        if !name.is_empty() {
            partitions.push(format!("/dev/{}", name));
        }
    }

    partitions
}

// expects /dev/sdaX or similar
pub fn get_partition_label(partition: &str) -> String {
    let name = Path::new(partition)
        .file_name()
        .and_then(|s| s.to_str())
        .expect(&format!("Could not determine file name from partition path: {}", partition));
    let uevent_path = format!("/sys/class/block/{}/uevent", name);
    let content = std::fs::read_to_string(&uevent_path)
        .expect(&format!("Failed to read uevent file at: {}", uevent_path));
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("PARTNAME=") {
            return rest.to_string();
        }
    }
    String::new()
}

// outputs /dev/sda2
pub fn get_partition(label: &str) -> String {
    let path = format!("/dev/disk/by-partlabel/{}", label);
    std::fs::canonicalize(&path)
        .expect(&format!("Failed to canonicalize partition label path: {}", path))
        .to_string_lossy()
        .into_owned()
}

// TODO: hacky, idk if it will work always
// outputs for example /dev/sda2
pub fn get_partition_by_numb(disk: &str, part_number: usize) -> String {
    if let Some(last_char) = disk.chars().last() {
        if last_char.is_numeric() {
            return format!("{}p{}", disk, part_number);
        }
    }

    format!("{}{}", disk, part_number)
}

use regex::Regex;
// Receives /dev/sda2
pub fn get_disk_part_numb(partition: &str) -> (String, u16) {
    let re = Regex::new(r"^(/dev/[a-zA-Z0-9]+(?:\d+)?(?:n\d+)?)(\d+)$")
        .expect("Failed to compile partition regex");
    re.captures(partition)
        .and_then(|cap| {
            let disk = cap.get(1)?.as_str().trim_end_matches('p').to_string();
            let num = cap.get(2)?.as_str().parse::<u16>().ok()?;
            Some((disk, num))
        })
        .expect(&format!("Failed to extract disk and partition number from path: {}", partition))
}

// Receives /dev/sda2
// Outputs start / size in sectors
pub fn get_sectors(partition: &str) -> (usize, usize) {
    let (disk, _part_number) = get_disk_part_numb(partition);
    let start_path = format!(
        "/sys/block/{}/{}/start",
        disk.replace("/dev/", "").replace("/", "/"),
        partition.replace("/dev/", "").replace("/", "/")
    );
    let mut start_str = read_file_str(start_path.clone())
        .expect(&format!("Failed to read sector start file: {}", start_path));
    
    start_str = start_str.replace("\n", "");
    let start = start_str.parse::<usize>()
        .expect(&format!("Failed to parse start sector value: {}", start_str));

    let size_path = format!(
        "/sys/block/{}/{}/size",
        disk.replace("/dev/", "").replace("/", "/"),
        partition.replace("/dev/", "").replace("/", "/")
    );
    let size = read_file_str(size_path.clone())
        .expect(&format!("Failed to read sector size file: {}", size_path))
        .replace("\n", "")
        .parse::<usize>()
        .expect("Failed to parse size sector value");
        
    (start, size)
}

// accepts labels
pub fn remove_partition(label: &str) {
    let (disk, part_number) = get_disk_part_numb(&get_partition(label));
    show_wait_toast(&format!(
        "Remove partition of label: {} number: {} in disk: {}",
        label, part_number, disk
    ));
    run_command(&format!("parted {} -s rm {}", disk, part_number), true)
        .expect(&format!("Failed to remove partition {} on disk {}", part_number, disk));
}

pub fn move_partition_left(label: &str) {
    let partition = get_partition(label);
    let (disk, part_number) = get_disk_part_numb(&partition);

    show_wait_toast(&format!(
        "Resize partition of label: {} of number: {} in disk: {} to the left",
        label, part_number, disk
    ));

    let mut c = 1;
    let mut previous_partition = get_partition_by_numb(&disk, (part_number - c) as usize);
    while !path_exists(&previous_partition) {
        c = c + 1;
        previous_partition = get_partition_by_numb(&disk, (part_number - c) as usize);
    }
    info!("Final previous partition is: {}", previous_partition);

    let (previous_start, previous_size) = get_sectors(&previous_partition);
    let (_start, size) = get_sectors(&partition);
    let new_start_sector = previous_start + previous_size + 1;
    let _new_end_sector = new_start_sector + size;
    run_shell_command(
        &format!(
            "echo '{},{}' | sfdisk --move-data {} -N {}",
            new_start_sector, size, disk, part_number
        ),
        true,
    )
    .expect(&format!("Failed to move data using sfdisk on disk {} partition {}", disk, part_number));

    run_command(&format!("e2fsck -f -y -v -C 0 {}", &partition), true)
        .expect(&format!("Failed to run e2fsck on partition {}", partition));
}

// mb are *1000 from gb!
// only ext4
pub fn resize_partition(label: &str, size_mb: usize) {
    let partition = get_partition(label);
    let (disk, part_number) = get_disk_part_numb(&partition);

    show_wait_toast(&format!(
        "Resize partition of label: {} number: {} in disk: {} to size {}M",
        label, part_number, disk, size_mb
    ));

    info!("Running e2fsck");
    run_command(&format!("e2fsck -f -y -C 0 {}", &partition), true)
        .expect(&format!("e2fsck failed for {}", partition));

    info!("Running resize2fs");
    run_command(&format!("resize2fs -p {} {}M", &partition, size_mb), true)
        .expect(&format!("resize2fs failed for partition {}", partition));

    info!("Running parted");
    let (start, _size) = get_sectors(&partition);
    let new_size = (size_mb * 1024 * 1024) / BLOCK_SIZE;
    run_shell_command(
        &format!(
            "echo Yes | parted {} ---pretend-input-tty unit s resizepart {} {}",
            disk,
            part_number,
            start + new_size
        ),
        true,
    )
    .expect(&format!("parted resizepart failed for partition {} on disk {}", part_number, disk));

    info!("Running partprobe");
    run_command(&format!("partprobe {}", disk), true)
        .expect(&format!("partprobe failed for disk {}", disk));

    info!("Running e2fsck");
    run_command(&format!("e2fsck -f -y -C 0 {}", &partition), true)
        .expect(&format!("Final e2fsck failed for partition {}", partition));
}

/*
// For resizing, nope
    run_command(
        &format!("e2fsck -f -y -C 0 {}", get_partition(label)),
        true,
    )
    .unwrap();

    let (start, _size) = get_sectors(&get_partition(label));
    let new_size = (size_mb * 1024 * 1024) / BLOCK_SIZE;
    run_shell_command(
        &format!(
            "echo '{},{}' | sfdisk --move-data {} -N {}",
            start, new_size, disk, part_number
        ),
        true,
    )
    .unwrap();

    run_command(
        &format!("e2fsck -f -y -v -C 0 {}", get_partition(label)),
        true,
    )
    .unwrap();
*/

pub fn create_partition(after_label: &str, size_mb: usize, new_label: &str) {
    show_wait_toast(&format!(
        "Create partition after label: {} size: {}M new_label {}",
        after_label, size_mb, new_label
    ));

    let previous_partition = get_partition(after_label);
    let (disk, _previous_part_number) = get_disk_part_numb(&previous_partition);
    let (previous_start, previous_size) = get_sectors(&previous_partition);
    let new_start = previous_start + previous_size;
    let new_size = (size_mb * 1024 * 1024) / BLOCK_SIZE;
    let new_end = new_start + new_size;
    run_shell_command(
        &format!(
            "echo Yes | parted --align optimal -s {} unit s mkpart {} ext4 {} {}",
            disk, new_label, new_start, new_end
        ),
        true,
    )
    .expect(&format!("Failed to create partition {} on disk {}", new_label, disk));

    sleep_millis(500);
    run_command(&format!("partprobe {}", disk), true)
        .expect("partprobe failed");
    sleep_millis(500);

    sleep_millis(500);
    run_command(&format!("mkfs.ext4 {}", get_partition(new_label)), true)
        .expect("mkfs.ext4 failed");
    sleep_millis(500);

    sleep_millis(500);
    run_command(&format!("partprobe {}", disk), true)
        .expect("Final partprobe failed");
    sleep_millis(500);
}

pub fn get_partition_usage(partition: &str) -> u32 {
    let output = Command::new("df")
        .arg("-m")
        .arg(format!("/dev/{}", partition))
        .output()
        .expect(&format!("Failed to run df for partition: {}", partition));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().nth(1)
        .expect(&format!("df output for /dev/{} lacked a second line", partition));
    
    line.split_whitespace().nth(2)
        .expect("Could not find usage column in df output")
        .parse()
        .expect("Failed to parse disk usage as number")
}
