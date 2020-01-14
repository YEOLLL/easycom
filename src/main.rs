extern crate serialport as serial;

use std::time::Duration;
use serial::prelude::*;
use clap::{Arg, App};

fn str_to_databits(str:&str) -> Result<DataBits, ()> {
    match str {
        "5" => Ok(DataBits::Five),
        "6" => Ok(DataBits::Six),
        "7" => Ok(DataBits::Seven),
        "8" => Ok(DataBits::Eight),
        _ => Err(()),
    }
}

fn str_to_flow_control(str:&str) -> Result<FlowControl, ()> {
    match str {
        "None" => Ok(FlowControl::None), // 无流量控制。
        "Software" => Ok(FlowControl::Software), // 使用XON / XOFF字节进行流控制。
        "Hardware" => Ok(FlowControl::Hardware), // 使用RTS / CTS信号进行流量控制。
        _ => Err(()),
    }
}

fn str_to_parity(str:&str) -> Result<Parity, ()> {
    match str {
        "None" => Ok(Parity::None), // 没有奇偶校验位。
        "Odd" => Ok(Parity::Odd), // 奇偶校验位设置1位的奇数。
        "Even" => Ok(Parity::Even), // 奇偶校验位设置1位的偶数。
        _ => Err(()),
    }
}

fn str_to_stop_bits(str:&str) -> Result<StopBits, ()> {
    match str {
        "1" => Ok(StopBits::One),
        "2" => Ok(StopBits::Two),
        _ => Err(()),
    }
}

fn main() {
    let matches = App::new("easycom")
        .version("0.1.0")
        .author("YEOL")
        .about("about应该怎么填......")
        .arg(Arg::with_name("device")
            .help("tty端口设备")
            .empty_values(false)
        )
        .arg(Arg::with_name("baud_rate")
            .short("b")
            .long("baud_rate")
            .help("波特率")
            .default_value("9600")
        )
        .arg(Arg::with_name("data_bits")
            .long("data_bits")
            .help("表示行上发送的字符的位数[5, 6, 7, 8]")
            .default_value("8")
        )
        .arg(Arg::with_name("flow_control")
            .long("flow_control")
            .help("控制数据传输的信令类型[None, Software, Hardware]")
            .default_value("None")
        )
        .arg(Arg::with_name("parity")
            .long("parity")
            .help("错误检查的奇偶校验类型[None, Odd, Even]")
            .default_value("None")
        )
        .arg(Arg::with_name("stop_bits")
            .long("stop_bits")
            .help("表示字符结尾的位数[1, 2]")
            .default_value("1")
        )
        .arg(Arg::with_name("timeout")
            .long("timeout")
            .help("等待接收数据的超时时间(毫秒)")
            .default_value("5")
        )
        .get_matches();

    if let Some(device) = matches.value_of("device") {
        let settings = SerialPortSettings {
            baud_rate: matches.value_of("baud_rate").unwrap().parse::<u32>()
                .expect("未知的baud_rate，请确认是否输入正确"),

            data_bits: str_to_databits(matches.value_of("data_bits").unwrap())
                .expect("未知的data_bits，请使用[5, 6, 7, 8]中的一个"),

            flow_control: str_to_flow_control(matches.value_of("flow_control").unwrap())
                .expect("未知的flow_control，请使用[None, Software, Hardware]中的一个"),

            parity: str_to_parity(matches.value_of("parity").unwrap())
                .expect("未知的parity，请使用[None, Odd, Even]中的一个"),

            stop_bits: str_to_stop_bits(matches.value_of("stop_bits").unwrap())
                .expect("未知的stop_bits，请使用[1, 2]中的一个"),

            timeout: Duration::from_millis(matches.value_of("timeout").unwrap().parse::<u64>()
                .expect("未知的timeout，请确认是否输入正确")),
        };
        let mut device = serial::open_with_settings(device, &settings)
                .expect("打开设备失败");
        let mut str = String::new();
        loop {
            device.read_to_string(&mut str).expect_err("读数据错误");
            println!("{}", str);
        }
    }
}