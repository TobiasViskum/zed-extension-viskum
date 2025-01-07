use zed_extension_api as zed;

struct ViskumExtension {}

impl zed::Extension for ViskumExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

zed::register_extension!(ViskumExtension);
