pub fn add_regexp_function(conn: &rusqlite::Connection) -> anyhow::Result<()> {
    let regexp = |ctx: &rusqlite::functions::Context| -> Result<bool, rusqlite::Error> {
        let mut case_insensitive = false;
        let mut ignore_whitespace = false;
        let mut swap_greed = false;
        let mut unicode = false;

        if ctx.len() == 3 {
            let flags_str = ctx
                .get_raw(2)
                .as_str()
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;

            let flags = flags_str.as_bytes();
            case_insensitive = flags.contains(&b'i');
            ignore_whitespace = flags.contains(&b'x');
            swap_greed = flags.contains(&b'U');
            unicode = flags.contains(&b'u');
        }

        let text = ctx
            .get_raw(1)
            .as_str()
            .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))?;

        let regexp: std::sync::Arc<lazy_regex::Regex> =
            ctx.get_or_create_aux(0, |vr| -> anyhow::Result<lazy_regex::Regex> {
                let re = lazy_regex::RegexBuilder::new(vr.as_str()?)
                    .case_insensitive(case_insensitive)
                    .ignore_whitespace(ignore_whitespace)
                    .swap_greed(swap_greed)
                    .unicode(unicode)
                    .build()?;
                Ok(re)
            })?;

        Ok(regexp.is_match(text))
    };

    conn.create_scalar_function(
        "regexp",
        2,
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            if ctx.len() != 2 {
                return Err(rusqlite::Error::InvalidParameterCount(2, ctx.len()));
            }

            let is_match = regexp(ctx)?;
            Ok(is_match)
        },
    )?;

    conn.create_scalar_function(
        "regexf",
        3,
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            if ctx.len() != 3 {
                return Err(rusqlite::Error::InvalidParameterCount(3, ctx.len()));
            }

            let is_match = regexp(ctx)?;
            Ok(is_match)
        },
    )?;

    Ok(())
}
