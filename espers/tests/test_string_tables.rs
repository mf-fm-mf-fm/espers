#[cfg(test)]
mod tests {
    use espers::string_table::StringTables;
    use glob::glob;
    use std::fs::read;
    use std::path::PathBuf;

    #[test]
    pub fn test_read_write_match() {
        let paths = glob("../assets/**/*.es[mp]")
            .unwrap()
            .into_iter()
            .map(|p| p.unwrap())
            .collect::<Vec<_>>();

        for path in paths {
            let language = "English";

            let mut string_tables = StringTables::new();
            let loaded =
                string_tables.load_plugin_path(&path.to_string_lossy().to_string(), language);

            if loaded.is_err() {
                continue;
            }

            for ((p, tt), table) in string_tables.tables {
                println!("{:?} - {:?}", path, tt);
                let plugin_path = PathBuf::from(&p);
                let plugin_stem = plugin_path.file_stem().unwrap().to_string_lossy();
                let serialized = table.serialize().unwrap();
                let raw_path = path.parent().unwrap().join(format!(
                    "Strings/{}_{}.{}",
                    plugin_stem,
                    language,
                    tt.extension()
                ));
                let raw = read(raw_path).unwrap();

                assert_eq!(serialized, raw);
            }
        }
    }
}
