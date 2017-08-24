//! A NonMaskable Interrupt (NMI) can be signalled by a peripheral or triggered
//! by software. This is the highest priority exception other than reset.

use drone::exception::Exception;
use drone::reg::{Delegate, Reg, Sreg, ValuePointer};
use drone::reg::rcc::{self, CirBits};

static mut NMI: Nmi = Nmi {
  rcc_cir: Reg::new(),
};

/// The exception routine data.
pub struct Nmi {
  rcc_cir: Sreg<rcc::Cir>,
}

/// The exception configuration data.
pub struct NmiConfig {
  pub rcc_cir: Sreg<rcc::Cir>,
}

/// The exception handler.
pub extern "C" fn handler() {
  unsafe { NMI.run() }
}

impl Exception for Nmi {
  type Config = NmiConfig;

  unsafe fn config(config: NmiConfig) {
    let data = &mut NMI;
    data.rcc_cir = config.rcc_cir;
  }

  fn run(&mut self) {
    let rcc_cir = self.rcc_cir.ptr();
    if rcc_cir.read().css() {
      rcc_cir.modify(|reg| reg.css_clear());
      panic!("HSE clock failure");
    } else {
      panic!("Unknown NMI");
    }
  }
}
