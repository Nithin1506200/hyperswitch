use hyperswitch_domain_models::address::{Address, AddressDetails, PhoneDetails};
use masking::Secret;
use router::types::{self, api, domain, storage::enums, PaymentAddress};

use crate::{
    connector_auth,
    utils::{self, ConnectorActions},
};

#[derive(Clone, Copy)]
struct BitpayTest;
impl ConnectorActions for BitpayTest {}
impl utils::Connector for BitpayTest {
    fn get_data(&self) -> api::ConnectorData {
        use router::connector::Bitpay;
        utils::construct_connector_data_old(
            Box::new(Bitpay::new()),
            types::Connector::Bitpay,
            api::GetToken::Connector,
            None,
        )
    }

    fn get_auth_token(&self) -> types::ConnectorAuthType {
        utils::to_connector_auth_type(
            connector_auth::ConnectorAuthentication::new()
                .bitpay
                .expect("Missing connector authentication configuration")
                .into(),
        )
    }

    fn get_name(&self) -> String {
        "bitpay".to_string()
    }
}

static CONNECTOR: BitpayTest = BitpayTest {};

fn get_default_payment_info() -> Option<utils::PaymentInfo> {
    Some(utils::PaymentInfo {
        address: Some(PaymentAddress::new(
            None,
            Some(Address {
                address: Some(AddressDetails {
                    first_name: Some(common_utils::types::NameType::get_unchecked(
                        "first".to_string(),
                    )),
                    last_name: Some(common_utils::types::NameType::get_unchecked(
                        "last".to_string(),
                    )),
                    line1: Some(Secret::new("line1".to_string())),
                    line2: Some(Secret::new("line2".to_string())),
                    city: Some("city".to_string()),
                    zip: Some(Secret::new("zip".to_string())),
                    country: Some(api_models::enums::CountryAlpha2::IN),
                    ..Default::default()
                }),
                phone: Some(PhoneDetails {
                    number: Some(Secret::new("9123456789".to_string())),
                    country_code: Some("+91".to_string()),
                }),
                email: None,
            }),
            None,
            None,
        )),
        ..Default::default()
    })
}

fn payment_method_details() -> Option<types::PaymentsAuthorizeData> {
    Some(types::PaymentsAuthorizeData {
        amount: 1,
        currency: enums::Currency::USD,
        payment_method_data: domain::PaymentMethodData::Crypto(domain::CryptoData {
            pay_currency: None,
            network: None,
        }),
        confirm: true,
        statement_descriptor_suffix: None,
        statement_descriptor: None,
        setup_future_usage: None,
        mandate_id: None,
        off_session: None,
        setup_mandate_details: None,
        // capture_method: Some(capture_method),
        browser_info: None,
        order_details: None,
        order_category: None,
        email: None,
        customer_name: None,
        payment_experience: None,
        payment_method_type: None,
        session_token: None,
        enrolled_for_3ds: false,
        related_transaction_id: None,
        router_return_url: Some(String::from("https://google.com/")),
        webhook_url: Some(String::from("https://google.com/")),
        complete_authorize_url: None,
        capture_method: None,
        customer_id: None,
        surcharge_details: None,
        request_incremental_authorization: false,
        metadata: None,
        authentication_data: None,
        customer_acceptance: None,
        ..utils::PaymentAuthorizeType::default().0
    })
}

// Creates a payment using the manual capture flow
#[actix_web::test]
async fn should_only_authorize_payment() {
    let response = CONNECTOR
        .authorize_payment(payment_method_details(), get_default_payment_info())
        .await
        .expect("Authorize payment response");
    let resp = response.clone().response.ok().unwrap();
    assert_eq!(response.status, enums::AttemptStatus::AuthenticationPending);
    let endpoint = match resp {
        types::PaymentsResponseData::TransactionResponse {
            redirection_data, ..
        } => Some(redirection_data),
        _ => None,
    };
    assert!(endpoint.is_some())
}

// Synchronizes a successful transaction.
#[actix_web::test]
async fn should_sync_authorized_payment() {
    let response = CONNECTOR
        .psync_retry_till_status_matches(
            enums::AttemptStatus::Authorized,
            Some(types::PaymentsSyncData {
                connector_transaction_id: types::ResponseId::ConnectorTransactionId(
                    "NPf27TDfyU5mhcTCw2oaq4".to_string(),
                ),
                ..Default::default()
            }),
            get_default_payment_info(),
        )
        .await
        .expect("PSync response");
    assert_eq!(response.status, enums::AttemptStatus::Charged);
}

// Synchronizes a expired transaction.
#[actix_web::test]
async fn should_sync_expired_payment() {
    let response = CONNECTOR
        .psync_retry_till_status_matches(
            enums::AttemptStatus::Authorized,
            Some(types::PaymentsSyncData {
                connector_transaction_id: types::ResponseId::ConnectorTransactionId(
                    "bUsFf4RjQEahjbjGcETRS".to_string(),
                ),
                ..Default::default()
            }),
            get_default_payment_info(),
        )
        .await
        .expect("PSync response");
    assert_eq!(response.status, enums::AttemptStatus::Failure);
}
