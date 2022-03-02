/// Macro to use in a doc string to display short important information.
macro_rules! warning {
    ($s:literal) => {
        concat!(
            r#"<span style="border:solid #D2991D 0px;border-radius:3px;background:#FFF5D6;color:black;font-size:0.8rem;padding-left:2px;padding-right:2px">"#,
            $s,
            "</span>"
        )
    };
}
