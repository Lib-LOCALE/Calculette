fn main() {
    slint_build::compile("ui/calculatrice.slint").expect("compilation de l’interface Slint");

    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=icons/icon.ico");
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set("ProductName", "Calculette");
        res.set(
            "FileDescription",
            "Calculette native standard et scientifique",
        );
        res.set("LegalCopyright", "Copyright (c) Lib-LOCALE");
        res.compile()
            .expect("intégration de l’icône Windows de Calculette");

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let res_obj = std::path::Path::new(&out_dir).join("resource.o");
        if res_obj.exists() {
            println!("cargo:rustc-link-arg={}", res_obj.display());
        }
    }
}
