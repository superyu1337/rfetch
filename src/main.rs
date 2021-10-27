use std::io::{Write, Result};
use sysinfo::{ProcessorExt, System, SystemExt};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn write(color: Option<Color>, input: &str) -> Result<()> {
    let bufwtr = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = bufwtr.buffer();
    buffer.set_color(ColorSpec::new().set_fg(color)).expect("failed to set color");
    write!(&mut buffer, "{}", input).expect("failed to write to buffer");
    bufwtr.print(&buffer)
}

fn write_column(input1: &str, input2: &str) {
    write(Some(Color::Cyan), input1).unwrap();
    write(Some(Color::White), input2).unwrap();
}

fn get_uptime_str(uptime: u64) -> String {
    let uptime_minutes = uptime / 60;
    let uptime_hours = uptime_minutes / 60;
    let uptime_days = uptime_hours / 24;
    format!("{}d {}h {}m {}s", uptime_days, uptime_hours % 24, uptime_minutes % 60, uptime % 60)
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu0 = &sys.processors()[0];
    let uptime_string = get_uptime_str(sys.uptime());
    let mem_string = format!("{} MB / {} MB", sys.used_memory() / 1000, sys.total_memory() / 1000);

    write(Some(Color::Magenta), "assel").unwrap();
    write(Some(Color::White), "@").unwrap();
    write(Some(Color::Magenta), &sys.host_name().unwrap()).unwrap();

    write_column("\nos        ", &sys.name().unwrap());
    write_column("\nuptime    ", &uptime_string);
    write_column("\ncpu       ", &cpu0.brand());
    write_column("\nkernel    ", &sys.kernel_version().unwrap());
    write_column("\nmemory    ", &mem_string);
}
