// Loop that takes an u8 value from the NVS and stores it +1.

// Logs:

// I (60) boot: Partition Table:
// I (64) boot: ## Label            Usage          Type ST Offset   Length
// I (71) boot:  0 nvs              WiFi data        01 02 00009000 00004000
// I (78) boot:  1 otadata          OTA data         01 00 0000d000 00002000
// I (86) boot:  2 phy_init         RF data          01 01 0000f000 00001000
// I (93) boot:  3 factory          factory app      00 00 00010000 0014d000
// I (101) boot:  4 ota_0            OTA app          00 10 00160000 0014d000
// I (108) boot:  5 ota_1            OTA app          00 11 002b0000 0014d000
// I (116) boot: End of partition table
// E (120) esp_image: image at 0x160000 has invalid magic byte (nothing flashed here?)
// E (129) boot: OTA app partition slot 0 is not bootable
// I (134) boot_comm: chip revision: 3, min. application chip revision: 0
// I (142) esp_image: segment 0: paddr=00010020 vaddr=3c050020 size=25670h (153200) map
// I (173) esp_image: segment 1: paddr=00035698 vaddr=3fc8a600 size=00d00h (  3328) load
// I (174) esp_image: segment 2: paddr=000363a0 vaddr=40380000 size=09c78h ( 40056) load
// I (186) esp_image: segment 3: paddr=00040020 vaddr=42000020 size=4a9fch (305660) map
// I (233) esp_image: segment 4: paddr=0008aa24 vaddr=40389c78 size=00850h (  2128) load
// I (237) boot: Loaded app from partition at offset 0x10000
// I (237) boot: Disabling RNG early entropy source...
// I (253) cpu_start: Pro cpu up.
// I (262) cpu_start: Pro cpu start user code
// I (262) cpu_start: cpu freq: 160000000
// I (262) cpu_start: Application information:
// I (265) cpu_start: Project name:     libespidf
// I (270) cpu_start: App version:      1
// I (275) cpu_start: Compile time:     Jun 19 2023 10:18:59
// I (281) cpu_start: ELF file SHA256:  0000000000000000...
// I (287) cpu_start: ESP-IDF:          3cec3a0-dirty
// I (292) cpu_start: Min chip rev:     v0.3
// I (297) cpu_start: Max chip rev:     v0.99 
// I (302) cpu_start: Chip rev:         v0.3
// I (306) heap_init: Initializing. RAM available for dynamic allocation:
// I (314) heap_init: At 3FC8C270 len 000504A0 (321 KiB): DRAM
// I (320) heap_init: At 3FCDC710 len 00002950 (10 KiB): STACK/DRAM
// I (326) heap_init: At 50000020 len 00001FE0 (7 KiB): RTCRAM
// I (333) spi_flash: detected chip: generic
// I (338) spi_flash: flash io: dio
// I (342) sleep: Configure to isolate all GPIO pins in sleep state
// I (348) sleep: Enable automatic switching of GPIO sleep configuration
// I (355) cpu_start: Starting scheduler.
// Got namespace 'config' from default partition
// NVS does not contain tag "test_u8"
// Tag not updated EspError(4359)

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_svc::nvs::*;

use std::{thread::sleep, time::Duration};

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let nvs_default_partition: EspNvsPartition<NvsDefault> =
        EspDefaultNvsPartition::take().unwrap();

    let test_namespace = "test_ns";
    let nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            println!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => panic!("Could't get namespace {:?}", e),
    };

    let mut delay_ms = 1000;
    loop {
        sleep(Duration::from_millis(delay_ms));
        delay_ms = delay_ms * 2; // just to do less flash writings.

        let tag_test_u8 = "test_u8";

        let is_tag_present = nvs.contains(tag_test_u8).unwrap();

        match is_tag_present {
            true => println!("NVS already contains tag {:?}", tag_test_u8),
            false => println!("NVS does not contain tag {:?}", tag_test_u8),
        };

        let new_value: u8 = if is_tag_present {
            let v: u8 = nvs.get_u8(tag_test_u8).unwrap().unwrap();
            println!("{:?} = {:?}", tag_test_u8, v);
            v + 1
        } else {
            0
        };

        match nvs.set_u8(tag_test_u8, new_value) {
            Ok(_) => println!("Tag updated"),
            Err(e) => println!("Tag not updated {:?}", e),
            // returns this -> Tag not updated EspError(4359) -> ESP_ERR_NVS_INVALID_HANDLE (0x1107): Handle has been closed or is NULL
        };
    }
}
