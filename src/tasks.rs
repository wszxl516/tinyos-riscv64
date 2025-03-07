use arrayvec::ArrayString;

use crate::arch::trap::{disable_irq_s, enable_irq_s};
use crate::common::readable::HumanReadable;
use crate::common::sleep::sleep_ms;
use crate::device::console::getc;
use crate::device::syscon::{poweroff, reboot};
use crate::mm::heap_status;
use crate::task;
use crate::{pr_notice, pr_warn, print};
#[optimize(none)]
pub fn demo1() -> ! {
    loop {
        for _ in 0..10 {
            sleep_ms(1000);
        }
    }
}

#[optimize(none)]
pub fn demo0() -> ! {
    loop {
        for _ in 0..10 {
            sleep_ms(100);
        }
    }
}
fn ps() {
    disable_irq_s();
    pr_warn!(
        "|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}\n",
        "name",
        "pid",
        "priority",
        "slice",
        "total",
        "state"
    );
    {
        task::each_task(|t| {
            pr_warn!(
                "|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}|{:<10}\n",
                t.name,
                t.id,
                t.priority,
                t.time_slice,
                t.total_time,
                t.state
            )
        });
    }
    enable_irq_s();
}

pub fn shell() -> ! {
    fn read_line<const N: usize>(buffer: &mut ArrayString<N>, echo: bool, prompt: &str) {
        if echo {
            print!("{}", prompt)
        }
        loop {
            match getc() {
                Some(ch) => {
                    if echo {
                        print!("{}", ch)
                    }
                    if ch == b'\n' as char || ch == b'\r' as char || buffer.is_full() {
                        break;
                    }
                    buffer.push(ch);
                }
                None => {}
            }
            sleep_ms(50);
        }
    }
    let mut line_buffer = ArrayString::<64>::new();
    loop {
        read_line(&mut line_buffer, true, "shell>> ");
        pr_notice!("\n");
        match line_buffer.as_str() {
            "ps" => ps(),
            "date" => match crate::arch::rtc::Time::get_time().to_string() {
                Ok(date) => pr_warn!("{}\n", date),
                Err(err) => pr_warn!("{}\n", err),
            },
            "free" => {
                let (total, used, free) = heap_status();
                pr_warn!("|{:<10}|{:<10}|{:<10}\n", "total", "used", "free");
                pr_warn!(
                    "|{:<10}|{:<10}|{:<10}\n",
                    total.readable(2),
                    used.readable(2),
                    free.readable(2)
                );
            }
            "poweroff" => poweroff(),
            "reboot" => reboot(),
            "raise" => {
                pr_warn!("raise a exception!\n");
                unsafe { core::arch::asm!("ld t0, 0({tmp})", tmp = in(reg) usize::MAX) }
            }
            _ => pr_warn!("usage: \n\tps date free poweroff reboot raise\n"),
        }
        line_buffer.clear();
        sleep_ms(100);
    }
}
