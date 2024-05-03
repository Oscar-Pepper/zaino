//! Integration tests for zingo-Proxy.
//! Currently uses ZCashD as ZebraD has not yet implemented Regtest Mode.

#![forbid(unsafe_code)]

use std::sync::{atomic::AtomicBool, Arc};
use zingo_netutils::GrpcConnector;
use zingo_testutils::{drop_test_manager, get_proxy_uri, launch_test_manager};

mod proxy {
    use super::*;

    #[tokio::test]
    async fn connect_to_lwd_get_info() {
        let online = Arc::new(AtomicBool::new(true));

        let (_regtest_manager, regtest_handles, _handles, proxy_port, _nym_addr) =
            launch_test_manager(online.clone()).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let proxy_uri = get_proxy_uri(proxy_port);
        println!("Attempting to connect to GRPC server at URI: {}", proxy_uri);

        // TODO: Add GrpcConnector that uses zingo-rpc's NymTxStreamerClient.
        let mut client = GrpcConnector::new(proxy_uri)
            .get_client()
            .await
            .expect("Failed to create GRPC client");

        let lightd_info = client
            .get_lightd_info(zcash_client_backend::proto::service::Empty {})
            .await
            .expect("Failed to retrieve lightd info from GRPC server");

        println!("{:#?}", lightd_info.into_inner());

        // TODO: Flush TempDir in drop_test_manager
        drop_test_manager(regtest_handles, online).await
    }
}

#[cfg(feature = "nym")]
mod nym {
    // TODO: Add Nym Tests.
}

#[cfg(feature = "darkside")]
mod darkside {
    // TODO: Add darkside tests.
}
