#![allow(dead_code)]

use std::marker::PhantomData;

use BuildStreamError;
use DefaultFormatError;
use DevicesError;
use DeviceNameError;
use Format;
use PauseStreamError;
use PlayStreamError;
use SupportedFormatsError;
use StreamData;
use SupportedFormat;

pub struct EventLoop;

impl EventLoop {
    #[inline]
    pub fn new() -> EventLoop {
        EventLoop
    }

    #[inline]
    pub fn run<F>(&self, _callback: F) -> !
        where F: FnMut(StreamId, StreamData)
    {
        loop { /* TODO: don't spin */ }
    }

    #[inline]
    pub fn build_input_stream(&self, _: &Device, _: &Format) -> Result<StreamId, BuildStreamError> {
        Err(BuildStreamError::DeviceNotAvailable)
    }

    #[inline]
    pub fn build_output_stream(&self, _: &Device, _: &Format) -> Result<StreamId, BuildStreamError> {
        Err(BuildStreamError::DeviceNotAvailable)
    }

    #[inline]
    pub fn destroy_stream(&self, _: StreamId) {
        unimplemented!()
    }

    #[inline]
    pub fn play_stream(&self, _: StreamId) -> Result<(), PlayStreamError> {
        panic!()
    }

    #[inline]
    pub fn pause_stream(&self, _: StreamId) -> Result<(), PauseStreamError> {
        panic!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamId;

#[derive(Default)]
pub struct Devices;

impl Devices {
    pub fn new() -> Result<Self, DevicesError> {
        Ok(Devices)
    }
}

impl Iterator for Devices {
    type Item = Device;

    #[inline]
    fn next(&mut self) -> Option<Device> {
        None
    }
}

#[inline]
pub fn default_input_device() -> Option<Device> {
    None
}

#[inline]
pub fn default_output_device() -> Option<Device> {
    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device;

impl Device {
    #[inline]
    pub fn supported_input_formats(&self) -> Result<SupportedInputFormats, SupportedFormatsError> {
        unimplemented!()
    }

    #[inline]
    pub fn supported_output_formats(&self) -> Result<SupportedOutputFormats, SupportedFormatsError> {
        unimplemented!()
    }

    #[inline]
    pub fn default_input_format(&self) -> Result<Format, DefaultFormatError> {
        unimplemented!()
    }

    #[inline]
    pub fn default_output_format(&self) -> Result<Format, DefaultFormatError> {
        unimplemented!()
    }

    #[inline]
    pub fn name(&self) -> Result<String, DeviceNameError> {
        Ok("null".to_owned())
    }
}

pub struct SupportedInputFormats;
pub struct SupportedOutputFormats;

impl Iterator for SupportedInputFormats {
    type Item = SupportedFormat;

    #[inline]
    fn next(&mut self) -> Option<SupportedFormat> {
        None
    }
}

impl Iterator for SupportedOutputFormats {
    type Item = SupportedFormat;

    #[inline]
    fn next(&mut self) -> Option<SupportedFormat> {
        None
    }
}

pub struct InputBuffer<'a, T: 'a> {
    marker: PhantomData<&'a T>,
}

pub struct OutputBuffer<'a, T: 'a> {
    marker: PhantomData<&'a mut T>,
}
