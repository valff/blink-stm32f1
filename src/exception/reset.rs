//! Invoked on power up and warm reset.

use VectorTable;
use consts::{PLL_MULT, SYS_TICK_SEC};
use drone::{itm, memory, util};
use drone::exception::ExceptionTable;
use drone::reg::{stk, AliasPointer, Delegate, Reg, Sreg, ValuePointer};
use drone::reg::flash::{self, AcrBits, AcrWaitStates};
use drone::reg::gpio::{self, CrMode, CrhPin};
use drone::reg::rcc::{self, Apb2enrBits, Apb2enrIop, CfgrBits,
                      CfgrPllHsePrescaler, CfgrPllSource, CfgrSystemClock,
                      CrBits};
use exception::{HardFaultConfig, NmiConfig, SysTickConfig};

/// The exception handler.
#[naked]
pub extern "C" fn handler() -> ! {
  // NOTE For each register delegate in this scope should exists exactly one
  // instance.
  let dbg_mcucr = Sreg::new();
  let dbg_demcr = Sreg::new();
  let dbg_tpiuspp = Sreg::new();
  let dbg_tpiuffc = Sreg::new();
  let dbg_itmla = Sreg::new();
  let dbg_itmtc = Sreg::new();
  let dbg_itmtp = Sreg::new();
  let stk_ctrl = Sreg::new();
  let stk_load = Sreg::new();
  let rcc_apb2enr = Sreg::new();
  let rcc_cr = Sreg::new();
  let rcc_cfgr = Sreg::new();
  let rcc_cir = Sreg::new();
  let flash_acr = Sreg::new();
  let gpio_ccrh = Sreg::new();
  let gpio_cbsrr = Sreg::new();

  unsafe {
    memory::bss_init();
    memory::data_init();
    itm::init(
      &dbg_mcucr,
      &dbg_demcr,
      &dbg_tpiuspp,
      &dbg_tpiuffc,
      &dbg_itmla,
      &dbg_itmtc,
      &dbg_itmtp,
    );
    clock_init(&rcc_cr, &rcc_cfgr, &flash_acr);
    peripheral_init(&rcc_apb2enr, &gpio_ccrh);
    VectorTable::config(move || {
      (
        NmiConfig { rcc_cir },
        HardFaultConfig {},
        SysTickConfig { gpio_cbsrr },
      )
    });
    exceptions_init(&stk_ctrl, &stk_load);
  }

  loop {
    util::wait_for_interrupt();
  }
}

unsafe fn clock_init<A, B, C>(
  rcc_cr: &Reg<rcc::Cr, A>,
  rcc_cfgr: &Reg<rcc::Cfgr, B>,
  flash_acr: &Reg<flash::Acr, C>,
) {
  let rcc_cr = rcc_cr.ptr();
  let rcc_cfgr = rcc_cfgr.ptr();

  // Enable HSE
  rcc_cr.modify(|reg| {
    reg.hse_enable(true).hse_bypass(false).css_enable(true)
  });
  while !rcc_cr.read().hse_ready() {}

  // Enable PLL
  rcc_cfgr.modify(|reg| {
    reg
      .pll_source(CfgrPllSource::Hse)
      .pll_hse_prescaler(CfgrPllHsePrescaler::None)
      .pll_multiplication(PLL_MULT)
  });
  rcc_cr.modify(|reg| reg.pll_enable(true));
  while !rcc_cr.read().pll_ready() {}

  // Configure flash latency
  flash_acr.ptr().modify(|reg| {
    reg
      .prefetch_enable(true)
      .half_cycle(false)
      .latency(AcrWaitStates::Two)
  });

  // Switch the system clock
  rcc_cfgr.modify(|reg| reg.system_clock(CfgrSystemClock::Pll));
}

unsafe fn peripheral_init<A, B>(
  rcc_apb2enr: &Reg<rcc::Apb2enr, A>,
  gpio_ccrh: &Reg<gpio::Crh<gpio::port::C>, B>,
) {
  rcc_apb2enr.ptr().bits().port_enable(Apb2enrIop::C, true);
  gpio_ccrh
    .ptr()
    .modify(|reg| reg.pin_mode(CrhPin::P13, CrMode::OutGpPuPu10));
}

unsafe fn exceptions_init<A, B>(
  stk_ctrl: &Reg<stk::Ctrl, A>,
  stk_load: &Reg<stk::Load, B>,
) {
  stk_load.ptr().write(|reg| reg.value(SYS_TICK_SEC / 2048));
  stk_ctrl.ptr().modify(|reg| reg.enable(true).tick(true));
}
