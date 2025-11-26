use zed_extension_api as zed;

struct CsoundExtension;

impl zed::Extension for CsoundExtension {
    fn new() -> Self
        where
            Self: Sized {
                Self
    }
}

zed::register_extension!(CsoundExtension);
