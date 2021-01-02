extern crate prettytable;
extern crate regex;

use clap::Clap;
use cli::Opts;
use config::use_config_or_opts;
use info::{cpu, gpu, memory, os, packages, wm, x};
use prettytable::{color, format, Attr, Cell, Row, Table};
use util::Result;

mod cli;
mod config;
mod info;
mod util;

// static ARCH_ASCII_ART: &str = "
//                    -`
//                   .o+`
//                  `ooo/
//                 `+oooo:
//                `+oooooo:
//                -+oooooo+:
//              `/:-:++oooo+:
//             `/++++/+++++++:
//            `/++++++++++++++:
//           `/+++ooooooooooooo/`
//          ./ooosssso++osssssso+`
//         .oossssso-````/ossssss+`
//        -osssssso.      :ssssssso.
//       :osssssss/        osssso+++.
//      /ossssssss/        +ssssooo/-
//    `/ossssso+/:-        -:/+osssso+-
//   `+sso+:-`                 `.-/+oso:
//  `++:.                           `-/+/
//  .`                                 `/";

fn create_cell(name: &str, data: String) -> Row {
    let mut name_cell = Cell::new(name)
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(color::BLUE));
    name_cell.align(format::Alignment::RIGHT);

    let mut value_cell = Cell::new(data.as_str());
    value_cell.align(format::Alignment::LEFT);

    Row::new(vec![name_cell, value_cell])
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let config = use_config_or_opts(&opts)?;

    let mut table = Table::new();
    table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(1, 1)
            .build(),
    );

    table.add_row(Row::new(vec![Cell::new(
        format!("{}@{}", os::get_user()?, os::get_hostname()?).as_str(),
    )
    .with_style(Attr::Bold)
    .with_style(Attr::ForegroundColor(color::MAGENTA))]));
    table.add_empty_row();

    if config.distro {
        let distro = os::get_distro_info()?;
        table.add_row(create_cell("distro", distro));
    }
    if config.kernel {
        let kernel = os::get_kernel_info()?;
        table.add_row(create_cell("kernel", kernel));
    }
    if config.shell {
        let shell = os::get_shell()?;
        table.add_row(create_cell("shell", shell));
    }
    if config.uptime {
        let uptime = os::get_uptime_info()?;
        table.add_row(create_cell("uptime", uptime));
    }
    if config.term {
        let term = os::get_terminal()?;
        table.add_row(create_cell("term", term));
    }
    if config.pacman {
        let packages = packages::get_packages_info()?;
        table.add_row(create_cell("pacman", packages.to_string()));
    }
    if config.wm {
        let wm = wm::get_wm_info()?;
        table.add_row(create_cell("wm", wm));
    }
    if config.resolution {
        let res = x::get_resolution()?;
        table.add_row(create_cell("res", res));
    }
    if config.cpu {
        let cpu = cpu::get_cpu_info()?;
        table.add_row(create_cell("cpu", cpu));
    }
    if config.mem {
        let memory = memory::get_memory_info()?;
        table.add_row(create_cell("mem", memory));
    }
    if config.gpu {
        let gpu = gpu::get_gpu_info()?;
        table.add_row(create_cell("gpu", gpu.join(", ")));
    }
    if config.theme {
        let gtk_theme = x::get_gtk_theme()?;
        table.add_row(create_cell("theme", gtk_theme));
    }
    if config.icons {
        let gtk_icon = x::get_gtk_icon()?;
        table.add_row(create_cell("icons", gtk_icon));
    }

    table.printstd();

    Ok(())
}
