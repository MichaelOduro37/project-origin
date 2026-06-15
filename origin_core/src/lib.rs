pub mod cipher;
pub mod tensegrity;
pub mod network;
pub mod immune;
pub mod qga;
pub mod telemetry;
pub mod daemon;
pub mod updater;

pub mod ui;

#[cfg(test)]
mod tests;
pub mod quorum;
pub mod crispr;
pub mod fermion;
pub mod curvature;
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod jni_export {
    use jni::objects::JClass;
    use jni::JNIEnv;
    use std::thread;

    #[no_mangle]
    pub extern "system" fn Java_com_example_originapp_MainActivity_startDaemon(
        _env: JNIEnv,
        _class: JClass,
    ) {
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                crate::daemon::run().await;
            });
        });
    }
}

// trigger rebuild

// trigger rebuild for responsiveness
pub mod snn;
pub mod hologram;
pub mod physarum;
