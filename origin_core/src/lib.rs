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
pub mod reservoir;
pub mod rmt;
pub mod sinkhorn;
pub mod network_coding;
pub mod vcg_auction;
pub mod proof_carrying_data;
pub mod compressed_sensing;
pub mod causal_inference;
pub mod category_theory;
pub mod complexity_sync;
pub mod immune_nsa;
pub mod active_inference;
pub mod topology_tda;
pub mod autocatalytic_raf;
pub mod constructal_routing;
pub mod information_bottleneck;
pub mod federated_smpc_ai;
pub mod mean_field_games;
pub mod sparse_memory;
pub mod turing_patterns;
pub mod metabolic_scaling;
pub mod percolation;
pub mod epigenetics;
pub mod kuramoto;
pub mod transformation_optics;
pub mod topological_insulator;
pub mod bose_einstein_condensate;
pub mod hawking_radiation;
pub mod dirac_antimatter;
pub mod quantum_teleportation;
pub mod photonic_firewall;
pub mod calabi_yau;
pub mod relativity;
pub mod quantum_tunneling;
pub mod minkowski;
pub mod no_cloning;
pub mod reversible_computing;
pub mod penrose_tiling;
pub mod time_crystal;
pub mod ribosome_vm;
pub mod cherenkov;
pub mod quantum_zeno;
pub mod sonoluminescence;
pub mod qcd_confinement;
pub mod strange_attractor;
pub mod spin_ice;
pub mod baryogenesis;
pub mod casimir_effect;
pub mod panspermia;
pub mod m_theory;
pub mod horizontal_gene_transfer;
pub mod neuroplasticity;
pub mod mycorrhizal_network;
pub mod symbiogenesis;
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
