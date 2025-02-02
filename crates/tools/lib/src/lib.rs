use std::collections::BTreeMap;

pub enum CallingConvention {
    Stdcall(usize),
    Cdecl,
}

pub fn default_metadata() -> Vec<metadata::File> {
    vec![
        metadata::File::new(
            std::include_bytes!("../../../libs/metadata/default/Windows.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/metadata/default/Windows.Win32.winmd").to_vec(),
        )
        .expect("invalid winmd"),
        metadata::File::new(
            std::include_bytes!("../../../libs/metadata/default/Windows.Wdk.winmd").to_vec(),
        )
        .expect("invalid winmd"),
    ]
}

/// Returns the libraries and their function and stack sizes used by the gnu and msvc tools to build the umbrella libs.
pub fn libraries() -> BTreeMap<String, BTreeMap<String, CallingConvention>> {
    let files = default_metadata();
    let reader = &metadata::Reader::new(&files);
    let mut libraries = BTreeMap::<String, BTreeMap<String, CallingConvention>>::new();

    for method in reader
        .namespaces()
        .flat_map(|namespace| reader.namespace_functions(namespace))
    {
        let library = reader.method_def_module_name(method);
        let impl_map = reader
            .method_def_impl_map(method)
            .expect("ImplMap not found");
        let flags = reader.impl_map_flags(impl_map);
        let name = reader.impl_map_import_name(impl_map).to_string();
        if flags.contains(metadata::PInvokeAttributes::CallConvPlatformapi) {
            let params = reader.method_def_size(method);
            libraries
                .entry(library)
                .or_default()
                .insert(name, CallingConvention::Stdcall(params));
        } else if flags.contains(metadata::PInvokeAttributes::CallConvCdecl) {
            libraries
                .entry(library)
                .or_default()
                .insert(name, CallingConvention::Cdecl);
        } else {
            unimplemented!();
        }
    }

    libraries
}
