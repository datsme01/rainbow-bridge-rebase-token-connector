use bridge_common::prover::{EthAddress, EthEventParams, EthRebaseEvent};
use ethabi::{ParamType, Token};
use hex::ToHex;
use near_sdk::{AccountId, Balance};

/// Data that was emitted by the Ethereum Rebase event.
/// @dev: u128 may underflow u256 eth param
/// TODO: Check erc20-bridge-token and erc20-connector for how amount is handled
#[derive(Debug, Eq, PartialEq)]
pub struct EthRebasedEvent {
    pub rebaser_address: EthAddress,
    pub token: String,
    pub epoch: u128,
    pub exchange_rate: Balance,
    pub cpi: Balance,
    pub requested_adjustment: Balance,
    pub timestamp: u128,
}

impl EthRebasedEvent {
    fn event_params() -> EthEventParams {
        vec![
            ("token".to_string(), ParamType::Address, true),
            ("epoch".to_string(), ParamType::Uint(256), true),
            ("exchange_rate".to_string(), ParamType::Uint(256), false),
            ("cpi".to_string(), ParamType::Uint(256), true),
            (
                "requested_adjustment".to_string(),
                ParamType::Int(256),
                false,
            ),
            ("timestamp".to_string(), ParamType::Uint(256), true),
        ]
    }

    /// Parse raw log entry data.
    pub fn from_log_entry_data(data: &[u8]) -> Self {
        let event =
            EthRebaseEvent::from_log_entry_data("LogRebase", EthRebasedEvent::event_params(), data);
        let token = event.log.params[0].value.clone().to_address().unwrap().0;
        let token = (&token).encode_hex::<String>();
        let epoch = event.log.params[1]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let exchange_rate = event.log.params[2]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let cpi = event.log.params[3]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let requested_adjustment = event.log.params[4]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let timestamp = event.log.params[5]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        Self {
            rebaser_address: event.rebaser_address,
            token,
            epoch,
            exchange_rate,
            cpi,
            requested_adjustment,
            timestamp,
        }
    }
    
    pub fn to_log_entry_data(&self) -> Vec<u8> {
        EthRebaseEvent::to_log_entry_data(
            "LogRebase",
            EthRebaseEvent::event_params(),
            self.rebaser_address,
            vec![
                hex::decode(self.token.clone()).unwrap(),
                hex::decode(self.epoch.clone()).unwrap(),
            ],
            vec![
                Token::Uint(self.exchange_rate.into()),
                Token::Uint(self.requested_adjustment.into()),
            ],
        )
    }
}

impl std::fmt::Display for EthRebasedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token: {}; epoch: {}; exchange_rate: {}; cpi: {}; requested_adjustment: {}; timestamp: {}",
            self.token, self.epoch, self.exchange_rate, self.cpi, self.requested_adjustment, self.timestamp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_data() {
        let event_data = EthRebasedEvent {
            rebaser_address: [0u8; 20],
            token: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
            epoch: 101,
            exchange_rate: 1.2,
            cpi: 123,
            requested_adjustment: 123,
            timestamp: 1664465862,
        };
        let data = event_data.to_log_entry_data();
        let result = EthRebasedEvent::from_log_entry_data(&data);
        assert_eq!(result, event_data);
    }
}
