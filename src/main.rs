use regex::Regex;

fn get_qr_mode(s: &str) -> &'static str {
    let modes = vec![
        ("0001", r"^[0-9]+$"),
        ("0010", r"^[A-Za-z0-9 $%*+./:-]+$"),
        ("0100", r"^[\x00-\xFF]*$"),
        ("1000", r"^[^\x00-\x7F\xA1-\xDF]*$"),
    ];

    modes.iter()
        .find_map(|(mode_str, pattern)| {
            let re = Regex::new(pattern).unwrap();
            if re.is_match(s) {
                Some(*mode_str)
            } else {
                None
            }
        })
        .unwrap_or("0000")
}

#[derive(Clone, Copy)]
enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

fn get_qr_data_capacity(version: u32, ecl: ErrorCorrectionLevel) -> u32 {
    // Data capacities for QR code versions 1-40
    // Source: https://www.qrcode.com/en/about/version.html
    let capacities = [
        [19, 16, 13, 9], [34, 28, 22, 16], [55, 44, 34, 26], [80, 64, 48, 36],
        [108, 86, 62, 46], [136, 108, 86, 60], [156, 124, 100, 72], [194, 154, 122, 86],
        [232, 182, 152, 106], [274, 216, 180, 130], [324, 254, 210, 154], [370, 290, 244, 178],
        [428, 334, 292, 207], [461, 365, 308, 235], [523, 415, 348, 267], [589, 453, 390, 295],
        [647, 507, 434, 325], [721, 563, 486, 367], [795, 627, 554, 399], [861, 669, 604, 439],
        [932, 714, 652, 462], [1006, 782, 674, 496], [1094, 860, 746, 528], [1174, 914, 808, 574],
        [1276, 1000, 870, 610], [1370, 1062, 952, 666], [1468, 1128, 1020, 711], [1531, 1193, 1051, 751],
        [1631, 1267, 1141, 805], [1735, 1373, 1225, 868], [1843, 1455, 1313, 908], [1955, 1541, 1409, 982],
        [2071, 1631, 1501, 1030], [2191, 1725, 1601, 1112], [2306, 1812, 1700, 1168], [2434, 1914, 1828, 1228],
        [2566, 1992, 1921, 1283], [2702, 2102, 2051, 1351], [2812, 2216, 2180, 1421], [2956, 2334, 2303, 1500],
    ];

    match ecl {
        ErrorCorrectionLevel::L => capacities[version as usize - 1][0],
        ErrorCorrectionLevel::M => capacities[version as usize - 1][1],
        ErrorCorrectionLevel::Q => capacities[version as usize - 1][2],
        ErrorCorrectionLevel::H => capacities[version as usize - 1][3],
    }
}

fn get_qr_version(input_length: usize, mode: &str, ecl: ErrorCorrectionLevel) -> Option<u32> {
    (1..=40).find_map(|version| {
        let capacity = get_qr_data_capacity(version, ecl);
        let capacity = match mode {
            "0001" => capacity * 10 / 3, // Numeric
            "0010" => capacity * 5 / 3,  // Alphanumeric
            "0100" => capacity,          // Byte
            "1000" => capacity * 2 / 3,  // Kanji
            _ => return None,
        };
        if input_length <= capacity as usize {
            Some(version)
        } else {
            None
        }
    })
}

fn main() {
    let input = "Hello, QR Code World!";
    let mode = get_qr_mode(input);
    let ecl = ErrorCorrectionLevel::L;
    let input_length = match mode {
        "0001" => input.chars().count(),
        "0010" => input.chars().count(),
        "0100" => input.as_bytes().len(),
        "1000" => input.chars().count(),
        _ => panic!("Invalid QR mode"),
    };
    let version = get_qr_version(input_length, mode, ecl).unwrap_or_else(|| {
        panic!("Input data too long for any QR code version")
    });
    println!("QR Code Version: {}", version);
}