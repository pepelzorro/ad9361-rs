//! FIR filter configuration

use crate::bindings;
use paste::paste;

/// Parameters used to configure the Tx FIR filter
///
/// The [Default](#impl-Default) value of this type matches the values from the
/// [example
/// project](https://github.com/analogdevicesinc/no-OS/tree/master/projects/ad9361/src)
/// in the [no-OS](https://github.com/analogdevicesinc/no-OS) library.
#[derive(Clone, Copy, Debug)]
pub struct Ad9361TxFir(pub(crate) bindings::AD9361_TXFIRConfig);

impl From<Ad9361TxFir> for bindings::AD9361_TXFIRConfig {
    fn from(fir: Ad9361TxFir) -> Self {
        fir.0
    }
}

/// Parameters used to configure the Rx FIR filter
///
/// The [Default](#impl-Default) value of this type matches the values from the
/// [example
/// project](https://github.com/analogdevicesinc/no-OS/tree/master/projects/ad9361/src)
/// in the [no-OS](https://github.com/analogdevicesinc/no-OS) library.
#[derive(Clone, Copy, Debug)]
pub struct Ad9361RxFir(pub(crate) bindings::AD9361_RXFIRConfig);

impl From<Ad9361RxFir> for bindings::AD9361_RXFIRConfig {
    fn from(fir: Ad9361RxFir) -> Self {
        fir.0
    }
}

impl Default for Ad9361TxFir {
    fn default() -> Self {
        // BPF PASSBAND 3/20 fs to 1/4 fs
        Self(bindings::AD9361_TXFIRConfig {
            tx: 3,
            tx_gain: -6,
            tx_int: 1,
            tx_coef: [
                -4, -6, -37, 35, 186, 86, -284, -315, 107, 219, -4, 271, 558,
                -307, -1182, -356, 658, 157, 207, 1648, 790, -2525, -2553, 748,
                865, -476, 3737, 6560, -3583, -14731, -5278, 14819, 14819,
                -5278, -14731, -3583, 6560, 3737, -476, 865, 748, -2553, -2525,
                790, 1648, 207, 157, 658, -356, -1182, -307, 558, 271, -4, 219,
                107, -315, -284, 86, 186, 35, -37, -6, -4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            tx_coef_size: 64,
            tx_path_clks: [0, 0, 0, 0, 0, 0],
            tx_bandwidth: 0,
        })
    }
}

impl Default for Ad9361RxFir {
    fn default() -> Self {
        // BPF PASSBAND 3/20 fs to 1/4 fs
        Self(bindings::AD9361_RXFIRConfig {
            rx: 3,
            rx_gain: 0,
            rx_dec: 1,
            rx_coef: [
                -4, -6, -37, 35, 186, 86, -284, -315, 107, 219, -4, 271, 558,
                -307, -1182, -356, 658, 157, 207, 1648, 790, -2525, -2553, 748,
                865, -476, 3737, 6560, -3583, -14731, -5278, 14819, 14819,
                -5278, -14731, -3583, 6560, 3737, -476, 865, 748, -2553, -2525,
                790, 1648, 207, 157, 658, -356, -1182, -307, 558, 271, -4, 219,
                107, -315, -284, 86, 186, 35, -37, -6, -4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            rx_coef_size: 64,
            rx_path_clks: [0, 0, 0, 0, 0, 0],
            rx_bandwidth: 0,
        })
    }
}

macro_rules! get_set_inner_value {
    ($o:ident, $(($property:ident, $type:ty, $doc:expr)),*) => {
        paste! {
            impl $o {
                $(
                    /// Builder method to set
                    #[doc = $doc]
                    #[must_use]
                    pub fn $property(mut self, value: $type) -> Self {
                        self.0.$property = value;
                        self
                    }
                    /// Get
                    #[doc = $doc]
                    pub fn [< get_ $property>](&self) -> $type {
                        self.0.$property
                    }
                )*
            }
        }
    };
}
macro_rules! get_set_inner_coefficents {
    ($o:ident, $property:ident, $doc:expr) => {
        paste! {
            impl $o {
                /// Builder method to set
                #[doc = $doc]
                #[must_use]
                ///
                /// # Panics
                ///
                /// Panics if the length of coefficients is greater than 128
                pub fn $property(mut self, coefficients: &[i16]) -> Self {
                    assert!(coefficients.len() <= 128);
                    let len = coefficients.len();
                    self.0.[< $property _size >] = len as u8;
                    self.0.$property[..len].copy_from_slice(coefficients);
                    self
                }
                /// Get
                #[doc = $doc]
                pub fn [< get_ $property>](&self) -> &[i16] {
                    let len = self.0.[< $property _size >] as usize;
                    &self.0.$property[..len]
                }

            }
        }
    };
}

get_set_inner_value!(
    Ad9361TxFir,
    (tx_gain, i32, "FIR Fixed Gain"),
    (tx_int, u32, "FIR Interpolation")
);
get_set_inner_coefficents!(Ad9361TxFir, tx_coef, "FIR Coefficients");
get_set_inner_value!(
    Ad9361RxFir,
    (rx_gain, i32, "FIR Fixed Gain"),
    (rx_dec, u32, "FIR Decimation")
);
get_set_inner_coefficents!(Ad9361RxFir, rx_coef, "FIR Coefficients");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tx_coefficients() {
        let txfir = Ad9361TxFir::default();
        assert_eq!(
            txfir.get_tx_coef(),
            &[
                -4, -6, -37, 35, 186, 86, -284, -315, 107, 219, -4, 271, 558,
                -307, -1182, -356, 658, 157, 207, 1648, 790, -2525, -2553, 748,
                865, -476, 3737, 6560, -3583, -14731, -5278, 14819, 14819,
                -5278, -14731, -3583, 6560, 3737, -476, 865, 748, -2553, -2525,
                790, 1648, 207, 157, 658, -356, -1182, -307, 558, 271, -4, 219,
                107, -315, -284, 86, 186, 35, -37, -6, -4
            ]
        );
    }

    #[test]
    fn set_tx_coefficients() {
        let txfir = Ad9361TxFir::default().tx_coef(&[0x55; 128]);
        assert_eq!(txfir.get_tx_coef(), &[0x55; 128]);

        let txfir = Ad9361TxFir::default().tx_coef(&[0x55; 10]);
        assert_eq!(txfir.get_tx_coef(), &[0x55; 10]);
    }

    #[test]
    #[should_panic]
    fn set_tx_coefficients_too_long() {
        let _ = Ad9361TxFir::default().tx_coef(&[11; 129]);
    }
}
