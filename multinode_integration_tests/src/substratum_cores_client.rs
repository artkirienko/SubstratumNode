// Copyright (c) 2017-2019, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.
use hopper_lib::hopper::LiveCoresPackage;
use node_lib::json_masquerader::JsonMasquerader;
use node_lib::masquerader::Masquerader;
use serde_cbor;
use std::net::SocketAddr;
use sub_lib::cryptde::CryptDE;
use sub_lib::cryptde::Key;
use sub_lib::cryptde::PlainData;
use sub_lib::hopper::IncipientCoresPackage;
use substratum_client::SubstratumNodeClient;

pub struct SubstratumCoresClient<'a> {
    cryptde: &'a CryptDE,
    delegate: SubstratumNodeClient,
}

impl<'a> SubstratumCoresClient<'a> {
    pub fn new(socket_addr: SocketAddr, cryptde: &'a CryptDE) -> SubstratumCoresClient<'a> {
        SubstratumCoresClient {
            cryptde,
            delegate: SubstratumNodeClient::new(socket_addr),
        }
    }

    pub fn transmit_package(
        &mut self,
        incipient_cores_package: IncipientCoresPackage,
        masquerader: &JsonMasquerader,
        recipient_key: Key,
    ) {
        let (live_cores_package, _) =
            LiveCoresPackage::from_incipient(incipient_cores_package, self.cryptde);
        let serialized_lcp = serde_cbor::ser::to_vec(&live_cores_package)
            .expect(format!("Serializing LCP: {:?}", live_cores_package).as_str());
        let encoded_serialized_package = self
            .cryptde
            .encode(&recipient_key, &PlainData::new(&serialized_lcp[..]))
            .unwrap();
        let masqueraded = masquerader
            .mask(&encoded_serialized_package.data[..])
            .expect(format!("Masquerading {}-byte serialized LCP", serialized_lcp.len()).as_str());
        self.delegate.send_chunk(masqueraded);
    }

    pub fn masquerade_live_cores_package(
        live_cores_package: LiveCoresPackage,
        masquerader: &JsonMasquerader,
    ) -> Vec<u8> {
        let serialized_lcp = serde_cbor::ser::to_vec(&live_cores_package)
            .expect(format!("Serializing LCP: {:?}", live_cores_package).as_str());
        masquerader
            .mask(&serialized_lcp[..])
            .expect(format!("Masquerading {}-byte serialized LCP", serialized_lcp.len()).as_str())
    }
}
