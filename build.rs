fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winres::WindowsResource::new();
        res.set("FileDescription", "Tool to debug unique identifiers (UUID, ULID, Snowflake, etc)");
        res.set("ProductName", "uuinfo");
        res.set("LegalCopyright", "Copyright (c) 2025 Ronaldo Ferreira");
        res.set("CompanyName", "Ronaldo Ferreira");
        res.set("InternalName", "uuinfo.exe");
        res.set("OriginalFilename", "uuinfo.exe");
        res.set("Comments", "https://github.com/racum/uuinfo");
        res.set_language(0x0409); // English (United States)
        res.set_icon("assets/uuinfo.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
