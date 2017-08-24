//! The vector table to interrupt service routines.

use drone::exception::{Exception, ExceptionTable, Handler, Reserved,
                       ResetHandler};
use exception::*;

/// The vector table.
pub struct VectorTable {
  pub reset: ResetHandler,
  pub nmi: Handler,
  pub hard_fault: Handler,
  pub mem_manage: Handler,
  pub bus_fault: Handler,
  pub usage_fault: Handler,
  pub _reserved0: [Reserved; 4],
  pub sv_call: Handler,
  pub debug_monitor: Handler,
  pub _reserved1: [Reserved; 1],
  pub pend_sv: Handler,
  pub sys_tick: Handler,

  // external interrupts start here
  pub _unspecified0: [Handler; 60],
}

impl VectorTable {
  /// Constructs a new `VectorTable`, and fills it with defined handlers.
  pub const fn new() -> VectorTable {
    VectorTable {
      reset: Some(reset::handler),
      nmi: Some(nmi::handler),
      hard_fault: Some(hard_fault::handler),
      mem_manage: None,
      bus_fault: None,
      usage_fault: None,
      _reserved0: [Reserved::Vector; 4],
      sv_call: None,
      debug_monitor: None,
      _reserved1: [Reserved::Vector; 1],
      pend_sv: None,
      sys_tick: Some(sys_tick::handler),
      _unspecified0: [None; 60],
    }
  }
}

impl ExceptionTable for VectorTable {
  type Config = (NmiConfig, HardFaultConfig, SysTickConfig);

  unsafe fn config<F>(f: F)
  where
    F: Send + 'static,
    F: FnOnce() -> Self::Config,
  {
    let config = f();
    Nmi::config(config.0);
    HardFault::config(config.1);
    SysTick::config(config.2);
  }
}
