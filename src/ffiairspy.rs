/* automatically generated by rust-bindgen */

pub type Enum_Unnamed1 = ::libc::c_uint;
pub const RECEIVER_MODE_OFF: ::libc::c_uint = 0;
pub const RECEIVER_MODE_RX: ::libc::c_uint = 1;
pub type receiver_mode_t = Enum_Unnamed1;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const AIRSPY_SAMPLERATE_10MSPS: ::libc::c_uint = 0;
pub const AIRSPY_SAMPLERATE_2_5MSPS: ::libc::c_uint = 1;
pub const AIRSPY_SAMPLERATE_END: ::libc::c_uint = 2;
pub type airspy_samplerate_t = Enum_Unnamed2;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const AIRSPY_INVALID: ::libc::c_uint = 0;
pub const AIRSPY_RECEIVER_MODE: ::libc::c_uint = 1;
pub const AIRSPY_SI5351C_WRITE: ::libc::c_uint = 2;
pub const AIRSPY_SI5351C_READ: ::libc::c_uint = 3;
pub const AIRSPY_R820T_WRITE: ::libc::c_uint = 4;
pub const AIRSPY_R820T_READ: ::libc::c_uint = 5;
pub const AIRSPY_SPIFLASH_ERASE: ::libc::c_uint = 6;
pub const AIRSPY_SPIFLASH_WRITE: ::libc::c_uint = 7;
pub const AIRSPY_SPIFLASH_READ: ::libc::c_uint = 8;
pub const AIRSPY_BOARD_ID_READ: ::libc::c_uint = 9;
pub const AIRSPY_VERSION_STRING_READ: ::libc::c_uint = 10;
pub const AIRSPY_BOARD_PARTID_SERIALNO_READ: ::libc::c_uint = 11;
pub const AIRSPY_SET_SAMPLERATE: ::libc::c_uint = 12;
pub const AIRSPY_SET_FREQ: ::libc::c_uint = 13;
pub const AIRSPY_SET_LNA_GAIN: ::libc::c_uint = 14;
pub const AIRSPY_SET_MIXER_GAIN: ::libc::c_uint = 15;
pub const AIRSPY_SET_VGA_GAIN: ::libc::c_uint = 16;
pub const AIRSPY_SET_LNA_AGC: ::libc::c_uint = 17;
pub const AIRSPY_SET_MIXER_AGC: ::libc::c_uint = 18;
pub const AIRSPY_MS_VENDOR_CMD: ::libc::c_uint = 19;
pub const AIRSPY_SET_RF_BIAS_CMD: ::libc::c_uint = 20;
pub const AIRSPY_GPIO_WRITE: ::libc::c_uint = 21;
pub const AIRSPY_GPIO_READ: ::libc::c_uint = 22;
pub const AIRSPY_GPIODIR_WRITE: ::libc::c_uint = 23;
pub const AIRSPY_GPIODIR_READ: ::libc::c_uint = 24;
pub const AIRSPY_GET_SAMPLERATES: ::libc::c_uint = 25;
pub const AIRSPY_GET_PACKING: ::libc::c_uint = 26;
pub type airspy_vendor_request = Enum_Unnamed3;
pub type Enum_Unnamed4 = ::libc::c_uint;
pub const GPIO_PORT0: ::libc::c_uint = 0;
pub const GPIO_PORT1: ::libc::c_uint = 1;
pub const GPIO_PORT2: ::libc::c_uint = 2;
pub const GPIO_PORT3: ::libc::c_uint = 3;
pub const GPIO_PORT4: ::libc::c_uint = 4;
pub const GPIO_PORT5: ::libc::c_uint = 5;
pub const GPIO_PORT6: ::libc::c_uint = 6;
pub const GPIO_PORT7: ::libc::c_uint = 7;
pub type airspy_gpio_port_t = Enum_Unnamed4;
pub type Enum_Unnamed5 = ::libc::c_uint;
pub const GPIO_PIN0: ::libc::c_uint = 0;
pub const GPIO_PIN1: ::libc::c_uint = 1;
pub const GPIO_PIN2: ::libc::c_uint = 2;
pub const GPIO_PIN3: ::libc::c_uint = 3;
pub const GPIO_PIN4: ::libc::c_uint = 4;
pub const GPIO_PIN5: ::libc::c_uint = 5;
pub const GPIO_PIN6: ::libc::c_uint = 6;
pub const GPIO_PIN7: ::libc::c_uint = 7;
pub const GPIO_PIN8: ::libc::c_uint = 8;
pub const GPIO_PIN9: ::libc::c_uint = 9;
pub const GPIO_PIN10: ::libc::c_uint = 10;
pub const GPIO_PIN11: ::libc::c_uint = 11;
pub const GPIO_PIN12: ::libc::c_uint = 12;
pub const GPIO_PIN13: ::libc::c_uint = 13;
pub const GPIO_PIN14: ::libc::c_uint = 14;
pub const GPIO_PIN15: ::libc::c_uint = 15;
pub const GPIO_PIN16: ::libc::c_uint = 16;
pub const GPIO_PIN17: ::libc::c_uint = 17;
pub const GPIO_PIN18: ::libc::c_uint = 18;
pub const GPIO_PIN19: ::libc::c_uint = 19;
pub const GPIO_PIN20: ::libc::c_uint = 20;
pub const GPIO_PIN21: ::libc::c_uint = 21;
pub const GPIO_PIN22: ::libc::c_uint = 22;
pub const GPIO_PIN23: ::libc::c_uint = 23;
pub const GPIO_PIN24: ::libc::c_uint = 24;
pub const GPIO_PIN25: ::libc::c_uint = 25;
pub const GPIO_PIN26: ::libc::c_uint = 26;
pub const GPIO_PIN27: ::libc::c_uint = 27;
pub const GPIO_PIN28: ::libc::c_uint = 28;
pub const GPIO_PIN29: ::libc::c_uint = 29;
pub const GPIO_PIN30: ::libc::c_uint = 30;
pub const GPIO_PIN31: ::libc::c_uint = 31;
pub type airspy_gpio_pin_t = Enum_Unnamed5;
pub type Enum_airspy_error = ::libc::c_int;
pub const AIRSPY_SUCCESS: ::libc::c_int = 0;
pub const AIRSPY_TRUE: ::libc::c_int = 1;
pub const AIRSPY_ERROR_INVALID_PARAM: ::libc::c_int = -2;
pub const AIRSPY_ERROR_NOT_FOUND: ::libc::c_int = -5;
pub const AIRSPY_ERROR_BUSY: ::libc::c_int = -6;
pub const AIRSPY_ERROR_NO_MEM: ::libc::c_int = -11;
pub const AIRSPY_ERROR_LIBUSB: ::libc::c_int = -1000;
pub const AIRSPY_ERROR_THREAD: ::libc::c_int = -1001;
pub const AIRSPY_ERROR_STREAMING_THREAD_ERR: ::libc::c_int = -1002;
pub const AIRSPY_ERROR_STREAMING_STOPPED: ::libc::c_int = -1003;
pub const AIRSPY_ERROR_OTHER: ::libc::c_int = -9999;
pub type Enum_airspy_board_id = ::libc::c_uint;
pub const AIRSPY_BOARD_ID_PROTO_AIRSPY: ::libc::c_uint = 0;
pub const AIRSPY_BOARD_ID_INVALID: ::libc::c_uint = 255;
pub type Enum_airspy_sample_type = ::libc::c_uint;
pub const AIRSPY_SAMPLE_FLOAT32_IQ: ::libc::c_uint = 0;
pub const AIRSPY_SAMPLE_FLOAT32_REAL: ::libc::c_uint = 1;
pub const AIRSPY_SAMPLE_INT16_IQ: ::libc::c_uint = 2;
pub const AIRSPY_SAMPLE_INT16_REAL: ::libc::c_uint = 3;
pub const AIRSPY_SAMPLE_UINT16_REAL: ::libc::c_uint = 4;
pub const AIRSPY_SAMPLE_END: ::libc::c_uint = 5;
pub enum Struct_airspy_device { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub device: *mut Struct_airspy_device,
    pub ctx: *mut ::libc::c_void,
    pub samples: *mut ::libc::c_void,
    pub sample_count: ::libc::c_int,
    pub sample_type: Enum_airspy_sample_type,
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type airspy_transfer_t = Struct_Unnamed6;
pub type airspy_transfer = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub part_id: [u32; 2usize],
    pub serial_no: [u32; 4usize],
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type airspy_read_partid_serialno_t = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub major_version: u32,
    pub minor_version: u32,
    pub revision: u32,
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type airspy_lib_version_t = Struct_Unnamed8;
pub type airspy_sample_block_cb_fn =
    ::std::option::Option<extern "C" fn(transfer: *mut airspy_transfer)
                              -> ::libc::c_int>;
#[link(name = "airspy")]
extern "C" {
    pub fn airspy_lib_version(lib_version: *mut airspy_lib_version_t) -> ();
    pub fn airspy_init() -> ::libc::c_int;
    pub fn airspy_exit() -> ::libc::c_int;
    pub fn airspy_open_sn(device: *mut *mut Struct_airspy_device,
                          serial_number: u64) -> ::libc::c_int;
    pub fn airspy_open(device: *mut *mut Struct_airspy_device)
     -> ::libc::c_int;
    pub fn airspy_close(device: *mut Struct_airspy_device) -> ::libc::c_int;
    pub fn airspy_get_samplerates(device: *mut Struct_airspy_device,
                                  buffer: *mut u32, len: u32)
     -> ::libc::c_int;
    pub fn airspy_set_samplerate(device: *mut Struct_airspy_device,
                                 samplerate: u32) -> ::libc::c_int;
    pub fn airspy_start_rx(device: *mut Struct_airspy_device,
                           callback: airspy_sample_block_cb_fn,
                           rx_ctx: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn airspy_stop_rx(device: *mut Struct_airspy_device) -> ::libc::c_int;
    pub fn airspy_is_streaming(device: *mut Struct_airspy_device)
     -> ::libc::c_int;
    pub fn airspy_si5351c_write(device: *mut Struct_airspy_device,
                                register_number: u8, value: u8)
     -> ::libc::c_int;
    pub fn airspy_si5351c_read(device: *mut Struct_airspy_device,
                               register_number: u8, value: *mut u8)
     -> ::libc::c_int;
    pub fn airspy_r820t_write(device: *mut Struct_airspy_device,
                              register_number: u8, value: u8)
     -> ::libc::c_int;
    pub fn airspy_r820t_read(device: *mut Struct_airspy_device,
                             register_number: u8, value: *mut u8)
     -> ::libc::c_int;
    pub fn airspy_gpio_write(device: *mut Struct_airspy_device,
                             port: airspy_gpio_port_t, pin: airspy_gpio_pin_t,
                             value: u8) -> ::libc::c_int;
    pub fn airspy_gpio_read(device: *mut Struct_airspy_device,
                            port: airspy_gpio_port_t, pin: airspy_gpio_pin_t,
                            value: *mut u8) -> ::libc::c_int;
    pub fn airspy_gpiodir_write(device: *mut Struct_airspy_device,
                                port: airspy_gpio_port_t,
                                pin: airspy_gpio_pin_t, value: u8)
     -> ::libc::c_int;
    pub fn airspy_gpiodir_read(device: *mut Struct_airspy_device,
                               port: airspy_gpio_port_t,
                               pin: airspy_gpio_pin_t, value: *mut u8)
     -> ::libc::c_int;
    pub fn airspy_spiflash_erase(device: *mut Struct_airspy_device)
     -> ::libc::c_int;
    pub fn airspy_spiflash_write(device: *mut Struct_airspy_device,
                                 address: u32, length: u16,
                                 data: *mut ::libc::c_uchar) -> ::libc::c_int;
    pub fn airspy_spiflash_read(device: *mut Struct_airspy_device,
                                address: u32, length: u16,
                                data: *mut ::libc::c_uchar) -> ::libc::c_int;
    pub fn airspy_board_id_read(device: *mut Struct_airspy_device,
                                value: *mut u8) -> ::libc::c_int;
    pub fn airspy_version_string_read(device: *mut Struct_airspy_device,
                                      version: *mut ::libc::c_char,
                                      length: u8) -> ::libc::c_int;
    pub fn airspy_board_partid_serialno_read(device:
                                                 *mut Struct_airspy_device,
                                             read_partid_serialno:
                                                 *mut airspy_read_partid_serialno_t)
     -> ::libc::c_int;
    pub fn airspy_set_sample_type(device: *mut Struct_airspy_device,
                                  sample_type: Enum_airspy_sample_type)
     -> ::libc::c_int;
    pub fn airspy_set_freq(device: *mut Struct_airspy_device,
                           freq_hz: u32) -> ::libc::c_int;
    pub fn airspy_set_lna_gain(device: *mut Struct_airspy_device,
                               value: u8) -> ::libc::c_int;
    pub fn airspy_set_mixer_gain(device: *mut Struct_airspy_device,
                                 value: u8) -> ::libc::c_int;
    pub fn airspy_set_vga_gain(device: *mut Struct_airspy_device,
                               value: u8) -> ::libc::c_int;
    pub fn airspy_set_lna_agc(device: *mut Struct_airspy_device,
                              value: u8) -> ::libc::c_int;
    pub fn airspy_set_mixer_agc(device: *mut Struct_airspy_device,
                                value: u8) -> ::libc::c_int;
    pub fn airspy_set_rf_bias(dev: *mut Struct_airspy_device, value: u8)
     -> ::libc::c_int;
    pub fn airspy_get_packing(device: *mut Struct_airspy_device,
                              value: *mut u8) -> ::libc::c_int;
    pub fn airspy_error_name(errcode: Enum_airspy_error)
     -> *const ::libc::c_char;
    pub fn airspy_board_id_name(board_id: Enum_airspy_board_id)
     -> *const ::libc::c_char;
}
