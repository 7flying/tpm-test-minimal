use anyhow::{Context, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    tpm_test_rhbz2220851()?;
    Ok(())
}

fn tpm_test_rhbz2220851() -> Result<()> {
    let tcti_conf = tss_esapi::tcti_ldr::TctiNameConf::Device(
        tss_esapi::tcti_ldr::DeviceConfig::from_str("/dev/tpmrm0").unwrap(),
    );
    let _tss_context =
        tss_esapi::Context::new(tcti_conf).context("Error initializing the TPM context")?;
    Ok(())
}
