pub static STYLE_BAR: std::sync::LazyLock<indicatif::ProgressStyle> =
    std::sync::LazyLock::new(|| {
        indicatif::ProgressStyle::with_template("[{elapsed_precise}] {wide_bar:.cyan/blue} {msg}")
            .unwrap()
    });
