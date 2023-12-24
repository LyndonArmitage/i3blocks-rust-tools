use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    seconds: u32,
}

#[derive(Debug)]
struct CPUData {
    used: u64,
    total: u64,
}

impl CPUData {
    fn from_path(path: &str) -> Self {
        let data = std::fs::read_to_string(path).unwrap();
        let mut user: u64 = 0;
        let mut nice: u64 = 0;
        let mut system: u64 = 0;
        let mut idle: u64 = 0;
        let mut iowait: u64 = 0;
        let mut irq: u64 = 0;
        let mut softirq: u64 = 0;
        let mut steal: u64 = 0;
        let mut guest: u64 = 0;
        let mut guest_nice: u64 = 0;
        for line in data.lines() {
            let mut parsed = false;
            let split = line.split_whitespace();
            for (i, txt) in split.enumerate() {
                if i == 0 {
                    if txt != "cpu" {
                        break;
                    }
                    parsed = true;
                    continue;
                }
                match i {
                    1 => user = txt.parse().unwrap(),
                    2 => nice = txt.parse().unwrap(),
                    3 => system = txt.parse().unwrap(),
                    4 => idle = txt.parse().unwrap(),
                    5 => iowait = txt.parse().unwrap(),
                    6 => irq = txt.parse().unwrap(),
                    7 => softirq = txt.parse().unwrap(),
                    8 => steal = txt.parse().unwrap(),
                    9 => guest = txt.parse().unwrap(),
                    10 => guest_nice = txt.parse().unwrap(),
                    _ => (),
                }
            }
            if parsed {
                break;
            }
        }

        let used = user + nice + system + irq + softirq + steal + guest + guest_nice;
        let total = used + idle + iowait;

        CPUData { total, used }
    }
}

const PATH: &str = "/proc/stat";

fn main() {
    let args = Args::parse();
    let sleep_duration = std::time::Duration::new(args.seconds as u64, 0);

    let mut last_value = CPUData::from_path(PATH);
    loop {
        let cpu = CPUData::from_path(PATH);
        let value: f64 =
            100f64 * (cpu.used - last_value.used) as f64 / (cpu.total - last_value.total) as f64;
        display(&value);
        last_value = cpu;
        std::thread::sleep(sleep_duration);
    }
}

fn display(value: &f64) {
    println!("CPU {:5.2}%", value);
}
