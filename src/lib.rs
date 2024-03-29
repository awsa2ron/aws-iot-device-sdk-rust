//! # Overview
//!
//! aws-iot-device-sdk (unofficial)
//!
//! The AWS IoT Device SDK is a collection of Rust source files, 
//! that can be used in embedded applications to securely connect IoT devices
//! to [AWS IoT Core](https://docs.aws.amazon.com/iot/latest/developerguide/what-is-aws-iot.html).
//!
//! It contains ~~MQTT client, HTTP client, JSON Parser,~~ AWS IoT Device Shadow, AWS IoT Jobs, 
//! and AWS IoT Device Defender libraries. 
//!
//! This SDK is distributed in source form, and can be built into customer firmware along with 
//! application code, other libraries and an operating system (OS) of your choice. 
//!
//! These libraries are only dependent on pure Rust libraries, so they can be ported to various OS's - from embedded 
//! Real Time Operating Systems (RTOS) to Linux/Mac/Windows. 
//!
//!
//!
// #![no_std]
pub mod backoff_algo;
pub mod common;
pub mod defender;
pub mod jobs;
pub mod shadow;
pub mod tunneling;

pub use common::*;

#[derive(Debug, PartialEq)]
pub enum TopicType {
    Other = 0,
    NamedShadow,
    Shadow,
    Jobs,
    Defender,
    Tunneling,
}
/// Given the topic string of an incoming message, determine whether it is
/// related to a device topic;
///
/// If it is, return the type of topic, like shadow ,jobs and so on.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::{TopicType, match_topic_type};
///
/// let topic = "$aws/things/chloe/shadow/name/common/get/rejected";
/// let topic_type = match_topic_type(topic).unwrap();
///
/// assert_eq!(topic_type, TopicType::NamedShadow);
/// ```
pub fn match_topic_type<'a>(topic: &'a str) -> Result<TopicType, Error> {
    is_valid_mqtt_topic(topic)?;

    let s = is_valid_prefix(topic, AWS_THINGS_PREFIX)?;

    let mid = s.find('/').ok_or(Error::NoMatch);
    let (thing_name, s) = s.split_at(mid?);
    is_valid_thing_name(thing_name)?;
    if s.starts_with(NAMED_SHADOW_API_BRIDGE)   { Ok(TopicType::NamedShadow) }
    else if s.starts_with(SHADOW_API_BRIDGE)    { Ok(TopicType::Shadow) }
    else if s.starts_with(JOBS_API_BRIDGE)      { Ok(TopicType::Jobs) }
    else if s.starts_with(DEFENDER_API_BRIDGE)  { Ok(TopicType::Defender) }
    else if s.starts_with(TUNNELS_API_BRIDGE)   { Ok(TopicType::Tunneling) }
    else { Err(Error::NoMatch) }
}
