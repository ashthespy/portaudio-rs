use ll;
use util::{to_pa_result, pa_time_to_duration};
use hostapi::HostApiIndex;
use pa::PaError;
use std::time::duration::Duration;
use std::c_str::CString;

pub type DeviceIndex = uint;

pub struct DeviceInfo
{
    pub name: String,
    pub host_api: HostApiIndex,
    pub max_input_channels: uint,
    pub max_output_channels: uint,
    pub default_low_input_latency: Duration,
    pub default_low_output_latency: Duration,
    pub default_high_input_latency: Duration,
    pub default_high_output_latency: Duration,
    pub default_sample_rate: f64,
}

impl DeviceInfo
{
    fn from_ll(input: &ll::PaDeviceInfo) -> DeviceInfo
    {
        DeviceInfo
        {
            name: format!("{}", unsafe { CString::new(input.name, false) }),
            host_api: input.hostApi as HostApiIndex,
            max_input_channels: input.maxInputChannels as uint,
            max_output_channels: input.maxOutputChannels as uint,
            default_low_input_latency: pa_time_to_duration(input.defaultLowInputLatency),
            default_low_output_latency: pa_time_to_duration(input.defaultLowOutputLatency),
            default_high_input_latency: pa_time_to_duration(input.defaultHighInputLatency),
            default_high_output_latency: pa_time_to_duration(input.defaultHighOutputLatency),
            default_sample_rate: input.defaultSampleRate,
        }
    }
}

pub fn get_count() -> Result<uint, PaError>
{
    match unsafe { ll::Pa_GetDeviceCount() }
    {
        n if n >= 0 => Ok(n as uint),
        m => to_pa_result(m).map(|_| 0),
    }
}

pub fn get_default_input_index() -> Result<DeviceIndex, PaError>
{
    match unsafe { ll::Pa_GetDefaultInputDevice() }
    {
        n if n >= 0 => Ok(n as uint),
        m => to_pa_result(m).map(|_| 0),
    }
}

pub fn get_default_output_index() -> Result<DeviceIndex, PaError>
{
    match unsafe { ll::Pa_GetDefaultOutputDevice() }
    {
        n if n >= 0 => Ok(n as uint),
        m => to_pa_result(m).map(|_| 0),
    }
}

pub fn get_info(index: DeviceIndex) -> Option<DeviceInfo>
{
    unsafe
    {
        ll::Pa_GetDeviceInfo(index as i32)
            .to_option()
            .map(|s| DeviceInfo::from_ll(s))
    }
}

pub fn get_from_host_api_device_index(host_api: HostApiIndex, host_api_device_index: uint) -> Result<DeviceIndex, PaError>
{
    match unsafe { ll::Pa_HostApiDeviceIndexToDeviceIndex(host_api as i32, host_api_device_index as i32) }
    {
        n if n >= 0 => Ok(n as DeviceIndex),
        m => to_pa_result(m).map(|_| 0),
    }
}