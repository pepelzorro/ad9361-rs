//! Initialisation Parameters

use crate::bindings;

/// Parameters used to configure the AD9361 RF PHY
///
/// The [Default](#impl-Default) value of this type matches the values from the
/// [example
/// project](https://github.com/analogdevicesinc/no-OS/tree/master/projects/ad9361/src)
/// in the [no-OS](https://github.com/analogdevicesinc/no-OS) library.
#[derive(Clone, Copy, Debug)]
pub struct Ad9361InitParam(pub(crate) bindings::AD9361_InitParam);

macro_rules! gettersetters {
    ($($section:expr =>
       [$($($doc:expr;)* pub $field:ident : $ty:ty,)+];
    )*) => {
        paste::paste! {
            $(
                #[doc=$section]
                impl Ad9361InitParam {
                    $(
                        // Getter
                        $( #[doc=$doc] )*
                        #[inline(always)]
                        pub fn [< $field:snake >](&self) -> $ty {
                            self.0.$field
                        }
                        // Setter
                        $( #[doc=$doc] )*
                        #[inline(always)]
                        pub fn [< set_ $field:snake >](&mut self, val: $ty) -> &mut Self {
                            self.0.$field = val;
                            self
                        }
                    )+
                }
            )*
        }
    }
}

gettersetters! {
    "Device Properties" => [
        pub id_no: u8,
        pub reference_clk_rate: u32,
    ];
    "Mode" => [
        pub two_rx_two_tx_mode_enable: u8,
        pub one_rx_one_tx_mode_use_rx_num: u8,
        pub one_rx_one_tx_mode_use_tx_num: u8,
        pub frequency_division_duplex_mode_enable: u8,
        pub frequency_division_duplex_independent_mode_enable: u8,
        pub tdd_use_dual_synth_mode_enable: u8,
        pub tdd_skip_vco_cal_enable: u8,
        pub tx_fastlock_delay_ns: u32,
        pub rx_fastlock_delay_ns: u32,
        pub rx_fastlock_pincontrol_enable: u8,
        pub tx_fastlock_pincontrol_enable: u8,
        pub external_rx_lo_enable: u8,
        pub external_tx_lo_enable: u8,
    ];
    "DC offset" => [
        pub dc_offset_tracking_update_event_mask: u8,
        pub dc_offset_attenuation_high_range: u8,
        pub dc_offset_attenuation_low_range: u8,
        pub dc_offset_count_high_range: u8,
        pub dc_offset_count_low_range: u8,
        pub split_gain_table_mode_enable: u8,
    ];
    "Clock tree" => [
        pub trx_synthesizer_target_fref_overwrite_hz: u32,
        pub qec_tracking_slow_mode_enable: u8,
        pub ensm_enable_pin_pulse_mode_enable: u8,
        pub ensm_enable_txnrx_control_enable: u8,
        pub rx_synthesizer_frequency_hz: u64,
        pub tx_synthesizer_frequency_hz: u64,
        pub tx_lo_powerdown_managed_enable: u8,
        pub rx_path_clock_frequencies: [u32; 6usize],
        pub tx_path_clock_frequencies: [u32; 6usize],
        pub rf_rx_bandwidth_hz: u32,
        pub rf_tx_bandwidth_hz: u32,
        pub rx_rf_port_input_select: u32,
        pub tx_rf_port_input_select: u32,
        pub tx_attenuation_mdB: i32,
        pub update_tx_gain_in_alert_enable: u8,
        pub xo_disable_use_ext_refclk_enable: u8,
        pub dcxo_coarse_and_fine_tune: [u32; 2usize],
        pub clk_output_mode_select: u32,
    ];
    "Gain control" => [
        pub gc_rx1_mode: u8,
        pub gc_rx2_mode: u8,
        pub gc_adc_large_overload_thresh: u8,
        pub gc_adc_ovr_sample_size: u8,
        pub gc_adc_small_overload_thresh: u8,
        pub gc_dec_pow_measurement_duration: u16,
        pub gc_dig_gain_enable: u8,
        pub gc_lmt_overload_high_thresh: u16,
        pub gc_lmt_overload_low_thresh: u16,
        pub gc_low_power_thresh: u8,
        pub gc_max_dig_gain: u8,
        pub gc_use_rx_fir_out_for_dec_pwr_meas_enable: u8,
    ];
    "Gain MGC Control" => [
        pub mgc_dec_gain_step: u8,
        pub mgc_inc_gain_step: u8,
        pub mgc_rx1_ctrl_inp_enable: u8,
        pub mgc_rx2_ctrl_inp_enable: u8,
        pub mgc_split_table_ctrl_inp_gain_mode: u8,
    ];
    "Gain AGC Control" => [
        pub agc_adc_large_overload_exceed_counter: u8,
        pub agc_adc_large_overload_inc_steps: u8,
        pub agc_adc_lmt_small_overload_prevent_gain_inc_enable: u8,
        pub agc_adc_small_overload_exceed_counter: u8,
        pub agc_dig_gain_step_size: u8,
        pub agc_dig_saturation_exceed_counter: u8,
        pub agc_gain_update_interval_us: u32,
        pub agc_immed_gain_change_if_large_adc_overload_enable: u8,
        pub agc_immed_gain_change_if_large_lmt_overload_enable: u8,
        pub agc_inner_thresh_high: u8,
        pub agc_inner_thresh_high_dec_steps: u8,
        pub agc_inner_thresh_low: u8,
        pub agc_inner_thresh_low_inc_steps: u8,
        pub agc_lmt_overload_large_exceed_counter: u8,
        pub agc_lmt_overload_large_inc_steps: u8,
        pub agc_lmt_overload_small_exceed_counter: u8,
        pub agc_outer_thresh_high: u8,
        pub agc_outer_thresh_high_dec_steps: u8,
        pub agc_outer_thresh_low: u8,
        pub agc_outer_thresh_low_inc_steps: u8,
        pub agc_attack_delay_extra_margin_us: u32,
        pub agc_sync_for_gain_counter_enable: u8,
    ];
    "Fast AGC" => [
        pub fagc_dec_pow_measuremnt_duration: u32,
        pub fagc_state_wait_time_ns: u32,
        pub fagc_allow_agc_gain_increase: u8,
        pub fagc_lp_thresh_increment_time: u32,
        pub fagc_lp_thresh_increment_steps: u32,
        pub fagc_lock_level_lmt_gain_increase_en: u8,
        pub fagc_lock_level_gain_increase_upper_limit: u32,
        pub fagc_lpf_final_settling_steps: u32,
        pub fagc_lmt_final_settling_steps: u32,
        pub fagc_final_overrange_count: u32,
        pub fagc_gain_increase_after_gain_lock_en: u8,
        pub fagc_gain_index_type_after_exit_rx_mode: u32,
        pub fagc_use_last_lock_level_for_set_gain_en: u8,
        pub fagc_rst_gla_stronger_sig_thresh_exceeded_en: u8,
        pub fagc_optimized_gain_offset: u32,
        pub fagc_rst_gla_stronger_sig_thresh_above_ll: u32,
        pub fagc_rst_gla_engergy_lost_sig_thresh_exceeded_en: u8,
        pub fagc_rst_gla_engergy_lost_goto_optim_gain_en: u8,
        pub fagc_rst_gla_engergy_lost_sig_thresh_below_ll: u32,
        pub fagc_energy_lost_stronger_sig_gain_lock_exit_cnt: u32,
        pub fagc_rst_gla_large_adc_overload_en: u8,
        pub fagc_rst_gla_large_lmt_overload_en: u8,
        pub fagc_rst_gla_en_agc_pulled_high_en: u8,
        pub fagc_rst_gla_if_en_agc_pulled_high_mode: u32,
        pub fagc_power_measurement_duration_in_state5: u32,
        pub fagc_large_overload_inc_steps: u32,
    ];
    "RSSI Control" => [
        pub rssi_delay: u32,
        pub rssi_duration: u32,
        pub rssi_restart_mode: u8,
        pub rssi_unit_is_rx_samples_enable: u8,
        pub rssi_wait: u32,
    ];
    "Aux ADC Control" => [
        pub aux_adc_decimation: u32,
        pub aux_adc_rate: u32,
    ];
    "AuxDAC Control" => [
        pub aux_dac_manual_mode_enable: u8,
        pub aux_dac1_default_value_mV: u32,
        pub aux_dac1_active_in_rx_enable: u8,
        pub aux_dac1_active_in_tx_enable: u8,
        pub aux_dac1_active_in_alert_enable: u8,
        pub aux_dac1_rx_delay_us: u32,
        pub aux_dac1_tx_delay_us: u32,
        pub aux_dac2_default_value_mV: u32,
        pub aux_dac2_active_in_rx_enable: u8,
        pub aux_dac2_active_in_tx_enable: u8,
        pub aux_dac2_active_in_alert_enable: u8,
        pub aux_dac2_rx_delay_us: u32,
        pub aux_dac2_tx_delay_us: u32,
    ];
    "Temperature Sensor Control" => [
        pub temp_sense_decimation: u32,
        pub temp_sense_measurement_interval_ms: u16,
        pub temp_sense_offset_signed: i8,
        pub temp_sense_periodic_measurement_enable: u8,
    ];
    "Control Out Setup" => [
        pub ctrl_outs_enable_mask: u8,
        pub ctrl_outs_index: u8,
        pub elna_settling_delay_ns: u32,
        pub elna_gain_mdB: u32,
        pub elna_bypass_loss_mdB: u32,
        pub elna_rx1_gpo0_control_enable: u8,
        pub elna_rx2_gpo1_control_enable: u8,
        pub elna_gaintable_all_index_enable: u8,
    ];
    "Digital Interface Control" => [
        pub digital_interface_tune_skip_mode: u8,
        pub digital_interface_tune_fir_disable: u8,
        pub pp_tx_swap_enable: u8,
        pub pp_rx_swap_enable: u8,
        pub tx_channel_swap_enable: u8,
        pub rx_channel_swap_enable: u8,
        pub rx_frame_pulse_mode_enable: u8,
        pub two_t_two_r_timing_enable: u8,
        pub invert_data_bus_enable: u8,
        pub invert_data_clk_enable: u8,
        pub fdd_alt_word_order_enable: u8,
        pub invert_rx_frame_enable: u8,
        pub fdd_rx_rate_2tx_enable: u8,
        pub swap_ports_enable: u8,
        pub single_data_rate_enable: u8,
        pub lvds_mode_enable: u8,
        pub half_duplex_mode_enable: u8,
        pub single_port_mode_enable: u8,
        pub full_port_enable: u8,
        pub full_duplex_swap_bits_enable: u8,
        pub delay_rx_data: u32,
        pub rx_data_clock_delay: u32,
        pub rx_data_delay: u32,
        pub tx_fb_clock_delay: u32,
        pub tx_data_delay: u32,
        pub lvds_bias_mV: u32,
        pub lvds_rx_onchip_termination_enable: u8,
        pub rx1rx2_phase_inversion_en: u8,
        pub lvds_invert1_control: u8,
        pub lvds_invert2_control: u8,
    ];
    "GPO Control" => [
        pub gpo_manual_mode_enable: u8,
        pub gpo_manual_mode_enable_mask: u32,
        pub gpo0_inactive_state_high_enable: u8,
        pub gpo1_inactive_state_high_enable: u8,
        pub gpo2_inactive_state_high_enable: u8,
        pub gpo3_inactive_state_high_enable: u8,
        pub gpo0_slave_rx_enable: u8,
        pub gpo0_slave_tx_enable: u8,
        pub gpo1_slave_rx_enable: u8,
        pub gpo1_slave_tx_enable: u8,
        pub gpo2_slave_rx_enable: u8,
        pub gpo2_slave_tx_enable: u8,
        pub gpo3_slave_rx_enable: u8,
        pub gpo3_slave_tx_enable: u8,
        pub gpo0_rx_delay_us: u8,
        pub gpo0_tx_delay_us: u8,
        pub gpo1_rx_delay_us: u8,
        pub gpo1_tx_delay_us: u8,
        pub gpo2_rx_delay_us: u8,
        pub gpo2_tx_delay_us: u8,
        pub gpo3_rx_delay_us: u8,
        pub gpo3_tx_delay_us: u8,
    ];
    "Tx Monitor Control" => [
        pub low_high_gain_threshold_mdB: u32,
        pub low_gain_dB: u32,
        pub high_gain_dB: u32,
        pub tx_mon_track_en: u8,
        pub one_shot_mode_en: u8,
        pub tx_mon_delay: u32,
        pub tx_mon_duration: u32,
        pub tx1_mon_front_end_gain: u32,
        pub tx2_mon_front_end_gain: u32,
        pub tx1_mon_lo_cm: u32,
        pub tx2_mon_lo_cm: u32,
    ];
}

impl Default for Ad9361InitParam {
    fn default() -> Self {
        let rx_path_clock_frequencies = [
            983040000, 245760000, 122880000, 61440000, 30720000, 30720000,
        ];
        let tx_path_clock_frequencies = [
            983040000, 122880000, 122880000, 61440000, 30720000, 30720000,
        ];

        let gpio_resetb = bindings::gpio_init_param {
            number: -1,
            ..Default::default()
        };
        let gpio_sync = bindings::gpio_init_param {
            number: -1,
            ..Default::default()
        };
        let gpio_cal_sw1 = bindings::gpio_init_param {
            number: -1,
            ..Default::default()
        };
        let gpio_cal_sw2 = bindings::gpio_init_param {
            number: -1,
            ..Default::default()
        };
        let spi_param = bindings::spi_init_param::default();

        Self(bindings::AD9361_InitParam {
            #[cfg(feature = "ad9361_device")]
            dev_sel: bindings::dev_id::ID_AD9361,
            #[cfg(feature = "ad9364_device")]
            dev_sel: bindings::dev_id::ID_AD9364,
            #[cfg(feature = "ad9363a_device")]
            dev_sel: bindings::dev_id::ID_AD9363A,
            // Reference Clock
            reference_clk_rate: 40_000_000,
            // Mode
            two_rx_two_tx_mode_enable: 1,
            one_rx_one_tx_mode_use_rx_num: 1,
            one_rx_one_tx_mode_use_tx_num: 1,
            frequency_division_duplex_mode_enable: 1,
            // DC offset
            dc_offset_tracking_update_event_mask: 5,
            dc_offset_attenuation_high_range: 6,
            dc_offset_attenuation_low_range: 5,
            dc_offset_count_high_range: 0x28,
            dc_offset_count_low_range: 0x32,
            // Clock tree
            trx_synthesizer_target_fref_overwrite_hz: 80_008_000, //80 MHz + 100ppm
            rx_synthesizer_frequency_hz: 2_400_000_000,
            tx_synthesizer_frequency_hz: 2_479_000_000,
            tx_lo_powerdown_managed_enable: 1,
            rx_path_clock_frequencies,
            tx_path_clock_frequencies,
            rf_rx_bandwidth_hz: 18_000_000,
            rf_tx_bandwidth_hz: 18_000_000,
            tx_attenuation_mdB: 10_000,
            dcxo_coarse_and_fine_tune: [8, 5920],
            // Gain control
            gc_rx1_mode: 2,
            gc_rx2_mode: 2,
            gc_adc_large_overload_thresh: 58,
            gc_adc_ovr_sample_size: 4,
            gc_adc_small_overload_thresh: 47,
            gc_dec_pow_measurement_duration: 8192,
            gc_lmt_overload_high_thresh: 800,
            gc_lmt_overload_low_thresh: 704,
            gc_low_power_thresh: 24,
            gc_max_dig_gain: 15,
            // Gain MGC Control
            mgc_dec_gain_step: 2,
            mgc_inc_gain_step: 2,
            // Gain AGC Control
            agc_adc_large_overload_exceed_counter: 10,
            agc_adc_large_overload_inc_steps: 2,
            agc_adc_small_overload_exceed_counter: 10,
            agc_dig_gain_step_size: 4,
            agc_dig_saturation_exceed_counter: 3,
            agc_gain_update_interval_us: 1000,
            agc_inner_thresh_high: 10,
            agc_inner_thresh_high_dec_steps: 1,
            agc_inner_thresh_low: 12,
            agc_inner_thresh_low_inc_steps: 1,
            agc_lmt_overload_large_exceed_counter: 10,
            agc_lmt_overload_large_inc_steps: 2,
            agc_lmt_overload_small_exceed_counter: 10,
            agc_outer_thresh_high: 5,
            agc_outer_thresh_high_dec_steps: 2,
            agc_outer_thresh_low: 18,
            agc_outer_thresh_low_inc_steps: 2,
            agc_attack_delay_extra_margin_us: 1,
            // Fast AGC
            fagc_dec_pow_measuremnt_duration: 64,
            fagc_state_wait_time_ns: 260,
            // Fast AGC - Low Power
            fagc_lp_thresh_increment_time: 5,
            fagc_lp_thresh_increment_steps: 1,
            // Fast AGC - Lock Level (Lock Level is set via slow AGC inner high threshold)
            fagc_lock_level_lmt_gain_increase_en: 1,
            fagc_lock_level_gain_increase_upper_limit: 5,
            // Fast AGC - Peak Detectors and Final Settling
            fagc_lpf_final_settling_steps: 1,
            fagc_lmt_final_settling_steps: 1,
            fagc_final_overrange_count: 3,
            // Fast AGC - Final Power Test
            // Fast AGC - Unlocking the Gain
            fagc_use_last_lock_level_for_set_gain_en: 1,
            fagc_rst_gla_stronger_sig_thresh_exceeded_en: 1,
            fagc_optimized_gain_offset: 5,
            fagc_rst_gla_stronger_sig_thresh_above_ll: 10,
            fagc_rst_gla_engergy_lost_sig_thresh_exceeded_en: 1,
            fagc_rst_gla_engergy_lost_goto_optim_gain_en: 1,
            fagc_rst_gla_engergy_lost_sig_thresh_below_ll: 10,
            fagc_energy_lost_stronger_sig_gain_lock_exit_cnt: 8,
            fagc_rst_gla_large_adc_overload_en: 1,
            fagc_rst_gla_large_lmt_overload_en: 1,
            fagc_power_measurement_duration_in_state5: 64,
            fagc_large_overload_inc_steps: 2,
            // RSSI Control
            rssi_delay: 1,
            rssi_duration: 1000,
            rssi_restart_mode: 3,
            rssi_wait: 1,
            // Aux ADC Control
            aux_adc_decimation: 256,
            aux_adc_rate: 40000000,
            // AuxDAC Control
            aux_dac_manual_mode_enable: 1,
            // Temperature Sensor Control
            temp_sense_decimation: 256,
            temp_sense_measurement_interval_ms: 1000,
            temp_sense_offset_signed: -49,
            temp_sense_periodic_measurement_enable: 1,
            // Control Out Setup
            ctrl_outs_enable_mask: 0xFF,
            // Digital Interface Control
            pp_tx_swap_enable: 1,
            pp_rx_swap_enable: 1,
            rx_frame_pulse_mode_enable: 1,
            lvds_mode_enable: 1,
            rx_data_delay: 4,
            tx_fb_clock_delay: 7,
            lvds_bias_mV: 150,
            lvds_rx_onchip_termination_enable: 1,
            lvds_invert1_control: 0xFF,
            lvds_invert2_control: 0x0F,
            // Tx Monitor Control
            low_high_gain_threshold_mdB: 37000,
            high_gain_dB: 24,
            tx_mon_delay: 511,
            tx_mon_duration: 8192,
            tx1_mon_front_end_gain: 2,
            tx2_mon_front_end_gain: 2,
            tx1_mon_lo_cm: 48,
            tx2_mon_lo_cm: 48,
            gpio_resetb,
            gpio_sync,
            gpio_cal_sw1,
            gpio_cal_sw2,
            spi_param,
            ..Default::default()
        })
    }
}
