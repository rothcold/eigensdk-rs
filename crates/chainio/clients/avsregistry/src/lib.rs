//! AvsRegistry methods for reading, writing and subscribing purposes.

use alloy_primitives::B256;

#[allow(dead_code)]
/// Reader module
pub mod reader;
#[allow(dead_code)]
/// Subscriber module
pub mod subscriber;

#[allow(dead_code)]
/// Writer module
pub mod writer;

/// Avs registry error message
pub mod error;

// /// cast sig-event "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))"
// pub const NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE: H256 = H256([
//     0xe3, 0xfb, 0x66, 0x13, 0xaf, 0x2e, 0x89, 0x30, 0xcf, 0x85, 0xd4, 0x7f, 0xcf, 0x6d, 0xb1, 0x01,
//     0x92, 0x22, 0x4a, 0x64, 0xc6, 0xcb, 0xe8, 0x02, 0x3e, 0x0e, 0xee, 0x1b, 0xa3, 0x82, 0x80, 0x41,
// ]);

// /// cast sig-event "OperatorSocketUpdate(bytes32,string)"
// pub const OPERATOR_SOCKET_UPDATE_EVENT_SIGNATURE: H256 = H256([
//     0xec, 0x29, 0x63, 0xab, 0x21, 0xc1, 0xe5, 0x0e, 0x1e, 0x58, 0x2a, 0xa5, 0x42, 0xaf, 0x2e, 0x4b,
//     0xf7, 0xbf, 0x38, 0xe6, 0xe1, 0x40, 0x3c, 0x27, 0xb4, 0x2e, 0x1c, 0x5d, 0x6e, 0x62, 0x1e, 0xaa,
// ]);
