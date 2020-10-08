use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let base = super::linux_base::opts();
    Target {
        llvm_target: "armv5te-unknown-linux-gnueabi".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
            features: "+soft-float,+strict-align".to_string(),
            // Atomic operations provided by compiler-builtins
            max_atomic_width: Some(32),
            unsupported_abis: super::arm_base::unsupported_abis(),
            target_mcount: "\u{1}__gnu_mcount_nc".to_string(),
            has_thumb_interworking: true,
            ..base
        },
    }
}
