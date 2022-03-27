//! An AD9361 RF PHY

use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

use embedded_hal::{blocking, digital};
use managed::ManagedSlice;
use paste::paste;

use crate::{bindings, fir::*, init, interop, types::*};

/// An AD9361 RF PHY
pub struct Ad9361<'a, SPI, DELAY, RESETB> {
    inner: *mut bindings::ad9361_rf_phy,
    params: init::Ad9361InitParam,
    is_init: bool,
    spi: SPI,
    delay: DELAY,
    resetb: Option<RESETB>,
    heap: ManagedSlice<'a, u32>,
    _pinned: core::marker::PhantomPinned,
}

// We use static pointers and a non-reentrant allocator to interact with the C
// driver. Therefore there must be at most one instance of AD9361 representation
// in existance at any one time
static TAKEN: AtomicBool = AtomicBool::new(false);

impl<'a, SPI, DELAY, RESETB> Ad9361<'a, SPI, DELAY, RESETB> {
    /// Attempt to free allocated memory in driver
    ///
    /// Returns true if memory was freed
    fn free_inner(&mut self) -> bool {
        if self.is_init && !self.inner.is_null() {
            let inner_ptr = self.inner;

            let _status = unsafe {
                cpp! ([
                    inner_ptr as "ad9361_rf_phy*"
                ] -> i32 as "int32_t"
                      {
                          return ad9361_remove(inner_ptr);
                      })
            }; // return status is always zero

            self.inner = ptr::null_mut();
            self.is_init = false;
            return true;
        }
        false
    }
    /// Exclusive access to the inner SPI peripheral
    pub fn inner_spi(&mut self) -> &mut SPI {
        &mut self.spi
    }
    /// Exclusive access to the inner delay
    pub fn inner_delay(&mut self) -> &mut DELAY {
        &mut self.delay
    }
}
impl<'a, SPI, DELAY, RESETB> Drop for Ad9361<'a, SPI, DELAY, RESETB> {
    fn drop(&mut self) {
        self.free_inner();
        assert!(TAKEN.swap(false, Ordering::AcqRel));
    }
}

impl<'a, SPI, DELAY, RESETB> Ad9361<'a, SPI, DELAY, RESETB>
where
    SPI: blocking::spi::Transfer<u8>,
    DELAY: blocking::delay::DelayMs<u32> + blocking::delay::DelayUs<u32>,
    RESETB: digital::v2::OutputPin,
{
    /// Construct new AD9361 representation
    ///
    /// # Panics
    ///
    /// Panics if an attempt is made to create a second AD9361 interface without
    /// dropping the first. Static pointers and a non-reentrant allocator are
    /// used to interact with the C driver, and thus there can be at most one
    /// instance in existance at a given time.
    pub fn new(
        spi: SPI,
        delay: DELAY,
        resetb: Option<RESETB>,
        heap: impl Into<ManagedSlice<'a, u32>>,
    ) -> Self {
        if TAKEN.swap(true, Ordering::AcqRel) {
            panic!("Attempt to create two AD9361 drivers simultaneously!");
        }

        Self {
            inner: ptr::null_mut(),
            params: init::Ad9361InitParam::default(),
            is_init: false,
            spi,
            delay,
            resetb,
            heap: heap.into(),
            _pinned: core::marker::PhantomPinned,
        }
    }

    /// Attempt to initialise a AD9361
    ///
    /// # Safety
    ///
    /// Self must not move after the call to `init()`. The `ad9361_rf_phy`
    /// structure in the C driver is self-referential
    pub fn init(
        &mut self,
        parameters: init::Ad9361InitParam,
    ) -> Result<(), i32> {
        self.params = parameters;

        // Set pointers to our wrapper methods and parts

        // SPI
        unsafe {
            self.params.0.spi_param.platform_ops =
                mem::transmute(interop::spi_wr_method::<SPI> as *mut ());
            self.params.0.spi_param.extra = mem::transmute(&self.spi);
        }
        // GPIO
        if let Some(resetb) = &self.resetb {
            unsafe {
                self.params.0.gpio_resetb.number = 1;
                self.params.0.gpio_resetb.platform_ops = mem::transmute(
                    interop::gpio_set_method::<RESETB> as *mut (),
                );
                self.params.0.gpio_resetb.extra = mem::transmute(&resetb);
            }
        }
        // Delay
        unsafe {
            interop::DELAY_MS =
                mem::transmute(interop::delay_ms_method::<DELAY> as *mut ());
            interop::DELAY_US =
                mem::transmute(interop::delay_us_method::<DELAY> as *mut ());
            interop::DELAY_OBJECT = mem::transmute(&self.delay);
        }
        // Heap
        unsafe {
            let (ptr, len) = match self.heap {
                ManagedSlice::Borrowed(ref mut slice) => {
                    (slice.as_mut_ptr(), slice.len())
                }
                #[cfg(feature = "std")]
                ManagedSlice::Owned(ref mut vec) => {
                    (vec.as_mut_ptr(), vec.capacity())
                }
            };
            interop::init_admalloc(ptr, len);
        }

        // Attempt to free any previous initialisation
        self.free_inner();

        // Library initialisation
        let inner_ptr = &self.inner;
        let params = &self.params.0;
        let status = unsafe {
            cpp! ([
                inner_ptr as "ad9361_rf_phy**",
                params as "AD9361_InitParam*"
            ] -> i32 as "int32_t"
                  {
                      return ad9361_init(inner_ptr, params);
                  })
        };
        self.is_init = true;

        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}

impl<'a, SPI, DELAY, RESETB> Ad9361<'a, SPI, DELAY, RESETB> {
    // -------- RX chain --------
    ad9361_method!(GET_SET: rx_rf_gain, channel: u8;
                   i32 => i32; "receive RF gain for the selected channel");
    ad9361_method!(GET_SET: rx_rf_bandwidth;
                   u32 => u32; "RX RF bandwidth");
    ad9361_method!(GET_SET: rx_sampling_freq;
                   u32 => u32; "RX sampling frequency");
    ad9361_method!(GET_SET: rx_lo_freq;
                   u64 => u64; "RX LO frequency");

    ad9361_method!(SET: set_rx_lo_int_ext;
                   lo: InternalExternalLO => u8; "Switch between internal and external LO");
    ad9361_method!(GET: get_rx_rssi, channel: u8;
                   bindings::rf_rssi => f32; "Get the RSSI for the selected channel.
Channel 0 = RX1, 1 = RX2 ");

    ad9361_method!(GET_SET: rx_gain_control_mode, channel: u8;
                   RfGainControlMode => u8; "gain control mode for the selected channel.
Channel 0 = RX1, 1 = RX2 ");
    ad9361_method!(SET: set_rx_fir_config;
                   config: Ad9361RxFir => bindings::AD9361_RXFIRConfig;
                   "Set the RX FIR configuration");
    ad9361_method!(GET_SET: rx_fir_en_dis;
                   bool > InBool => u8; "Enable/disable of the RX FIR filter");
    ad9361_method!(GET_SET: rx_rf_port_input;
                   RxRfPortSelection => u32; "selected RX RF input port");

    // -------- TX chain --------
    ad9361_method!(GET_SET: tx_attenuation, channel: u8;
                   u32 => u32; "transmit attenuation (in mdB) for the selected channel.
Channel 0 = TX1, 1 = TX2 ");
    ad9361_method!(GET_SET: tx_rf_bandwidth;
                   u32 => u32; "TX RF bandwidth");
    ad9361_method!(GET_SET: tx_sampling_freq;
                   u32 => u32; "TX sampling frequency");
    ad9361_method!(GET_SET: tx_lo_freq;
                   u64 => u64; "TX LO frequency");

    ad9361_method!(SET: set_tx_lo_int_ext;
                   lo: InternalExternalLO => u8; "Switch between internal and external LO");
    ad9361_method!(SET: set_tx_fir_config;
                   config: Ad9361TxFir => bindings::AD9361_TXFIRConfig;
                   "Set the TX FIR configuration");
    ad9361_method!(GET_SET: tx_fir_en_dis;
                   bool > InBool => u8; "Enable/disable of the TX FIR filter");

    ad9361_method!(GET_SET: tx_rf_port_output;
                   TxRfPortSelection => u32; "selected TX RF output port");

    ad9361_method!(SET: tx_lo_powerdown;
                   power: LOPowerStatus => u8; "Power down the TX Local Oscillator");
    ad9361_method!(GET: get_tx_lo_power;
                   u8 => LOPowerStatus; "Get the TX Local Oscillator power status");

    // -------- BIST --------
    ad9361_method!(GET_SET2: bist_prbs;
                   BistMode => bindings::ad9361_bist_mode;
                   "Built-in Self Test (BIST) Pseudo-Random Binary Sequence (PRBS) mode.");
    ad9361_method!(GET_SET2: bist_loopback;
                   LoopbackMode => i32;
                   "Built-in Self Test (BIST) loopback mode");
    ad9361_method!(SET: bist_tone;
                   mode: BistMode => bindings::ad9361_bist_mode,
                   frequency: u32, level_d_b: u32, mask: u32;
                   "Built-in Self Test (BIST) tone mode");

    // -------- Misc --------
    ad9361_method!(GET_INFALLIBLE_VAL: ensm_get_state;
                   u8 => EnsmState; "Get Enable State Machine (ENSM) state");
    ad9361_method!(GET: get_temperature;
                   i32 > TemperatureX1000 => f32; "Get the temperature in degrees Celsius");
    ad9361_method!(SET: tx_mute;
                   mute: bool => u32; "Mute transmit path.
Note that if you call `tx_mute(TxState::Unmute)` without ever calling `tx_mute(TxState::Mute)`,
then the TX gain will be set to -0 mdB");
}

/// Implementation of some methods from ad9361_conv.c
///
impl<'a, SPI, DELAY, RESETB> Ad9361<'a, SPI, DELAY, RESETB> {
    /// Set interface timing. Set `tx` for the TX path, clear `tx` for the RX
    /// path. If the `clock_delay` value has changed since the previous call or
    /// initial configuration, set `clock_changed`.
    ///
    /// # Panics
    ///
    /// Panics if `clock_delay` or `data_delay` are >= 16
    pub fn set_intf_delay(
        &mut self,
        tx: bool,
        clock_delay: u32,
        data_delay: u32,
        clock_changed: bool,
    ) -> Result<(), i32> {
        assert!(clock_delay < 16);
        assert!(data_delay < 16);

        assert!(
            !self.inner.is_null(),
            "Must call init() method before accessing ad9361"
        );
        let inner_ptr = self.inner;
        let status = unsafe {
            if clock_changed {
                let alert = EnsmState::Alert as u8;
                bindings::ad9361_ensm_force_state(inner_ptr, alert);
            }
            let address = if tx { 0x7 } else { 0x6 };
            let value = (clock_delay << 4) | data_delay;
            let status =
                bindings::ad9361_spi_write((*inner_ptr).spi, address, value);
            if clock_changed {
                let fdd = EnsmState::Fdd as u8;
                bindings::ad9361_ensm_force_state(inner_ptr, fdd);
            }
            status
        };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Set the LVDS bias control register 0x03C
    ///
    /// # Panics
    ///
    /// Panics if `lvds_bias_m_v` is < 75 or > 450
    pub fn set_lvds_bias_control(
        &mut self,
        rx_on_chip_term: bool,
        lvds_tx_lo_vcm: bool,
        lvds_bias_m_v: u32,
    ) -> Result<(), i32> {
        assert!(lvds_bias_m_v <= 450);
        assert!(lvds_bias_m_v >= 75);

        let address = 0x03C;
        let value = if rx_on_chip_term { 0x20 } else { 0 }
            | if lvds_tx_lo_vcm { 0x08 } else { 0 }
            | ((lvds_bias_m_v - 75) / 75);

        assert!(
            !self.inner.is_null(),
            "Must call init() method before accessing ad9361"
        );
        let inner_ptr = self.inner;
        let status = unsafe {
            bindings::ad9361_spi_write((*inner_ptr).spi, address, value)
        };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use embedded_hal::{blocking, digital};
    use serial_test::serial;

    use std::collections::HashMap;

    // Dummy reset pin, active low
    #[derive(Default)]
    struct DummyResetB {}
    impl digital::v2::OutputPin for DummyResetB {
        type Error = ();

        fn set_low(&mut self) -> Result<(), ()> {
            trace!("resetb asserted!");
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), ()> {
            trace!("resetb deasserted!");
            Ok(())
        }
    }

    // Dummy SPI interface that is actually a very shallow implementation of the
    // AD9361 register interface
    struct DummySPI {
        registers: HashMap<u16, u8>,
    }
    impl Default for DummySPI {
        fn default() -> DummySPI {
            let registers = HashMap::with_capacity(4096);
            DummySPI { registers }
        }
    }
    impl blocking::spi::Transfer<u8> for DummySPI {
        type Error = ();

        fn transfer<'w>(
            &mut self,
            words: &'w mut [u8],
        ) -> Result<&'w [u8], Self::Error> {
            let transaction = transaction::Ad9361Transaction(words);
            let register = transaction.register();
            let value = transaction.value();

            trace!("spi_transaction! {:?} {:x?}", transaction, words);

            if transaction.is_write() {
                // Save value
                self.registers.insert(register, value);
            } else {
                for i in 0..transaction.length() {
                    let reg = register + i as u16;
                    // Recall value (except for options below)
                    if let Some(value) = self.registers.get(&reg) {
                        // Recall
                        words[2 + i] = *value;
                    }
                }
            }

            // Product ID
            if register == 0x37 {
                words[2] = 0xA; // Rev[2:0] = 2
            }
            // BBPLL register
            if register == 0x0A {
                words[2] = 3; // default
            }
            // Temperature
            if register == 0xe {
                words[2] = 3;
            }
            // BB Cal register
            if register == 0x16 {
                words[2] = 0; // BB Cal always completes immediately
            }
            // Overflow register
            if register == 0x5e {
                words[2] = 0x80; // BBPLL always locks
            }
            // RxBBF
            if register == 0x1e6 {
                words[2] = 1; // default
            }
            if register == 0x1e8 || register == 0x1ea || register == 0x1ec {
                words[2] = 0x60; // default
            }
            // Rx Synth / Tx Synth
            if register == 0x244 || register == 0x284 {
                words[2] = 0xC0; // CP Cal is always valid and done
            }
            if register == 0x247 || register == 0x287 {
                words[2] = 0x02; // PLL always locks
            }

            Ok(words)
        }
    }

    #[test]
    fn struct_size() {
        let size = core::mem::size_of::<Ad9361InitParam>();
        println!("Ad9361InitParam {} bytes", size);
        assert!(size < 1024, "Ad9361 Init Param size has grown!");

        let size = core::mem::size_of::<
            Ad9361<DummySPI, DummyResetB, linux_embedded_hal::Delay>,
        >();
        println!("Ad9361 {} bytes", size);
        assert!(size < 1024, "Ad9361 size has grown!");
    }

    fn test_setup() -> (
        Ad9361InitParam,
        DummySPI,
        linux_embedded_hal::Delay,
        DummyResetB,
        Vec<u32>,
    ) {
        env_logger::try_init().ok();

        let parameters: Ad9361InitParam = Default::default();
        let spi: DummySPI = Default::default();
        let resetb: DummyResetB = Default::default();
        let delay = linux_embedded_hal::Delay {};
        let heap = Vec::with_capacity(540);

        (parameters, spi, delay, resetb, heap)
    }

    /// Basic initialisation
    #[test]
    #[serial]
    fn init() {
        let (parameters, spi, delay, resetb, heap) = test_setup();

        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();
    }

    /// Software reset (no dedicated reset pin)
    #[test]
    #[serial]
    fn software_reset() {
        let (parameters, spi, delay, _, heap) = test_setup();

        let mut ad9361: Ad9361<_, _, DummyResetB> =
            Ad9361::new(spi, delay, None, heap);
        ad9361.init(parameters).unwrap();
    }

    /// Re-initialise
    #[test]
    #[serial]
    fn reinit() {
        let (parameters, spi, delay, resetb, heap) = test_setup();

        let mut ad9361: Ad9361<_, _, DummyResetB> =
            Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        let parameters: Ad9361InitParam = Default::default();
        ad9361.init(parameters).unwrap(); // and again
    }

    /// Allocate the heap on the stack
    #[test]
    #[serial]
    fn static_heap() {
        let (parameters, spi, delay, resetb, _) = test_setup();
        let mut heap: [u32; 540] = [0; 540];

        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), &mut heap[..]);
        ad9361.init(parameters).unwrap();
    }

    /// Overflow the heap, check for panic
    #[test]
    #[serial]
    #[should_panic]
    fn overflow_heap() {
        let (parameters, spi, delay, resetb, _) = test_setup();
        let heap = Vec::with_capacity(400);

        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();
    }

    /// Don't call init method, check for panic
    #[test]
    #[serial]
    #[should_panic]
    fn init_skipped() {
        let (_parameters, spi, delay, resetb, heap) = test_setup();
        let ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);

        let _ = ad9361
            .get_temperature()
            .expect("Failed to read temperature");
    }

    /// Read the temperatures
    #[test]
    #[serial]
    fn temperature() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Read temperature");
        let t = ad9361
            .get_temperature()
            .expect("Failed to read temperature");
        info!("T = {:.1}ºC", t);
        info!("");

        assert!((t - 2.6).abs() < 0.1);
    }

    /// Configure BIST mode for the receive path
    #[test]
    #[serial]
    fn bist_prbs_rx() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Set PRBS");
        ad9361
            .bist_prbs(BistMode::InjectRx)
            .expect("Failed to set BIST mode");
    }

    /// Configure BIST mode for the transmit path
    #[test]
    #[serial]
    fn bist_loopback_tx() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Set Loopback");
        ad9361
            .bist_loopback(LoopbackMode::Enabled)
            .expect("Failed to set loopback mode");
    }

    /// Set the transmit attenuation value
    #[test]
    #[serial]
    fn tx_attenuation() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Set Tx Gain Attenuation");
        ad9361
            .set_tx_attenuation(1, 10_000)
            .expect("Failed to set Tx Gain Attenuation");
    }

    /// Power down the TX LO
    #[test]
    #[serial]
    fn powerdown_tx_lo() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Powerdown TX LO");
        ad9361
            .tx_lo_powerdown(LOPowerStatus::Off)
            .expect("Failed to powerdown TX LO");
        assert_eq!(
            ad9361
                .get_tx_lo_power()
                .expect("Failed to get power status of TX LO"),
            LOPowerStatus::Off
        );
    }

    /// Enable the TX FIR filter
    #[test]
    #[serial]
    fn tx_fir_filter_enable() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();
        let tx_fir = Ad9361TxFir::default();

        // must first set a value config
        ad9361.set_tx_fir_config(tx_fir).unwrap();

        info!("");
        info!("Enable TX FIR filter");
        assert!(!ad9361.get_tx_fir_en_dis().expect("Failed to get FIR en"));
        ad9361
            .set_tx_fir_en_dis(true)
            .expect("Failed to set FIR en");
        assert!(ad9361.get_tx_fir_en_dis().expect("Failed to get FIR en"));
    }

    /// Set the BBPLL and calculate Rx/Tx chain clocks
    #[test]
    #[serial]
    fn set_sampling_rate() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Set BB sampling rate");
        ad9361
            .set_rx_sampling_freq(4_000_000)
            .expect("Failed to set BB sampling rate");
    }

    /// Set the Rx and Tx Ports
    #[test]
    #[serial]
    fn set_rf_port_output() {
        let (parameters, spi, delay, resetb, heap) = test_setup();
        let mut ad9361 = Ad9361::new(spi, delay, Some(resetb), heap);
        ad9361.init(parameters).unwrap();

        info!("");
        info!("Set Ports Rx and Tx Ports");
        ad9361
            .set_rx_rf_port_input(RxRfPortSelection::B_BALANCED)
            .expect("Failed to set tx port");
        ad9361
            .set_tx_rf_port_output(TxRfPortSelection::TXB)
            .expect("Failed to set tx port");
    }
}
