pub fn server_config(
    host: String,
    root_cert: &rcgen::CertifiedKey,
    h2: bool,
) -> Result<rustls::ServerConfig, rustls::Error> {
    let mut cert_params = rcgen::CertificateParams::new(vec![host]).unwrap();
    cert_params
        .key_usages
        .push(rcgen::KeyUsagePurpose::DigitalSignature);
    cert_params
        .extended_key_usages
        .push(rcgen::ExtendedKeyUsagePurpose::ServerAuth);
    cert_params
        .extended_key_usages
        .push(rcgen::ExtendedKeyUsagePurpose::ClientAuth);

    let private_key = rcgen::KeyPair::generate().unwrap();

    let cert = cert_params
        .signed_by(&private_key, &root_cert.cert, &root_cert.key_pair)
        .unwrap();

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![rustls::pki_types::CertificateDer::from(cert)],
            rustls::pki_types::PrivateKeyDer::Pkcs8(rustls::pki_types::PrivatePkcs8KeyDer::from(
                private_key.serialize_der(),
            )),
        );

    if h2 {
        config.map(|mut server_config| {
            server_config.alpn_protocols = vec!["h2".into(), "http/1.1".into()];
            server_config
        })
    } else {
        config
    }
}
